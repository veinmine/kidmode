#!/usr/bin/env python3
"""Turn what the Horizon watch (.github/workflows/horizon-watch.yml) found into issues.

Reads the step results from the environment (PINNED, NEWEST, DATE, REGISTRY, FITS, DIFF,
UPDATE, TEST, TESTED, RUN) and the outputs in $REPORT, then keeps one open issue per label:

- horizon-compat: opened or updated while Kid Mode doesn't fit, build or pass its tests on
  the newest Horizon build; closed when it does again.
- horizon-api: opened or updated while the newest build changes an item Kid Mode uses, or
  adds, changes or removes one that a rule in .github/horizon-watchlist.txt matches; other
  API changes (nearly every Nightly has some) are left out. Closed when none is left.

An open issue is edited in place and gets a comment only when what it reports changes
(horizon-compat: a newer build still fails), so a weekly run doesn't notify twice. The standard library only.
"""

import hashlib
import json
import os
import re
import sys
import urllib.error
import urllib.request

API = os.environ.get("GITHUB_API_URL", "https://api.github.com")
REPO = os.environ.get("GITHUB_REPOSITORY", "")
TOKEN = os.environ.get("GITHUB_TOKEN", "")
SDK = "https://github.com/horizon-rust/horizon/releases/download"
REPORT = os.environ.get("REPORT", "report")
# GitHub refuses issue bodies and comments over 65536 characters.
LIMIT = 60000
KEY = re.compile(r"<!-- horizon-watch-key: (\S+) -->")

COMPAT = "horizon-compat"
API_LABEL = "horizon-api"
WATCHLIST = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "horizon-watchlist.txt")
# What `cargo horizon-mod update` prints when nothing the mod uses changed.
UNCHANGED = re.compile(r"none of the \d+ items it uses changed")
# An item line of `cargo horizon-mod sdk diff`: `- hook:camera_roll (added)`.
ITEM = re.compile(r"^- (\S+) \((added|changed|removed)\)", re.M)


def watch_hits(diff_text):
    """The diff's items a watchlist rule matches, as (item, change, feature)."""
    rules = []
    with open(WATCHLIST, encoding="utf-8") as f:
        for line in f:
            pattern, _, feature = line.partition("#")
            if pattern.strip():
                rules.append((re.compile(pattern.strip(), re.I), feature.strip()))
    hits = []
    for item, change in ITEM.findall(diff_text):
        for rule, feature in rules:
            if rule.search(item):
                hits.append((item, change, feature))
                break
    return hits


def call(method, path, data=None):
    req = urllib.request.Request(
        f"{API}/repos/{REPO}/{path}",
        method=method,
        data=None if data is None else json.dumps(data).encode(),
        headers={
            "Authorization": f"Bearer {TOKEN}",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        with urllib.request.urlopen(req) as res:
            text = res.read()
    except urllib.error.HTTPError as e:
        if method == "POST" and path == "labels" and e.code == 422:
            return None  # the label exists already
        sys.exit(f"{method} {path}: {e.code} {e.read().decode(errors='replace')}")
    return json.loads(text) if text else None


def clip(text):
    if len(text) <= LIMIT:
        return text
    return text[:LIMIT] + "\n\n…cut here: the whole report is in the run's horizon-watch-report artifact.\n"


def find(label):
    # The issues list returns pull requests too; skip them.
    for item in call("GET", f"issues?labels={label}&state=open&per_page=10") or []:
        if "pull_request" not in item:
            return item
    return None


def upsert(label, title, body, key, note):
    """Open the label's issue, or update the open one; comment `note` when `key` is new."""
    body = clip(f"<!-- horizon-watch-key: {key} -->\n{body}")
    issue = find(label)
    if not issue:
        call("POST", "labels", {"name": label, "color": "c5def5"})
        new = call("POST", "issues", {"title": title, "body": body, "labels": [label]})
        print(f"{label}: opened #{new['number']}")
        return
    old = KEY.search(issue.get("body") or "")
    call("PATCH", f"issues/{issue['number']}", {"title": title, "body": body})
    print(f"{label}: updated #{issue['number']}")
    if not old or old.group(1) != key:
        call("POST", f"issues/{issue['number']}/comments", {"body": note})
        print(f"{label}: commented on #{issue['number']}")


def close(label, comment):
    issue = find(label)
    if not issue:
        print(f"{label}: nothing open")
        return
    call("POST", f"issues/{issue['number']}/comments", {"body": comment})
    call("PATCH", f"issues/{issue['number']}", {"state": "closed", "state_reason": "completed"})
    print(f"{label}: closed #{issue['number']}")


def output(name, lines=None):
    """A report file's text, or its last `lines` lines; empty when the step made none."""
    try:
        with open(os.path.join(REPORT, name), encoding="utf-8", errors="replace") as f:
            text = f.read().strip()
    except FileNotFoundError:
        return ""
    if lines:
        text = "\n".join(text.splitlines()[-lines:])
    if len(text) > 40000:
        text = text[:40000] + "\n…cut here: the whole file is in the run's horizon-watch-report artifact."
    return text


def block(text):
    fence = "````" if "```" in text else "```"
    return f"{fence}text\n{text or '(no output)'}\n{fence}"


def surface_hash(build):
    """The mod API surface hash a build's SDK release records, or None."""
    try:
        with urllib.request.urlopen(f"{SDK}/sdk-{build}/sdk.json") as res:
            return json.load(res)["surface"]["hash"]
    except (urllib.error.URLError, KeyError, ValueError):
        return None


def mark(code):
    return {"0": "✅ passed", "": "⏭️ didn't run"}.get(code, f"❌ failed (exit {code})")


def main():
    pinned, newest, date = os.environ["PINNED"], os.environ["NEWEST"], os.environ["DATE"]
    fits, diff, update, test = (os.environ.get(k, "") for k in ("FITS", "DIFF", "UPDATE", "TEST"))
    tested = os.environ.get("TESTED", "")
    registry, run = os.environ.get("REGISTRY", "unknown"), os.environ.get("RUN", "")
    name = f"Nightly {date} (`{newest[:7]}`)"
    behind = pinned != newest

    head = (
        f"- Pinned build (`horizon-mod.toml`): `{pinned}`\n"
        f"- Newest Horizon build: {name}, `{newest}`\n"
        f"- Build the registry's checker is on: `{registry}`\n"
        f"- [Workflow run]({run}) (the full logs are its `horizon-watch-report` artifact)\n"
    )

    # 1. Compatibility.
    fits_word = {"0": "✅ fits", "1": "❌ doesn't fit", "2": "⚠️ couldn't tell"}.get(fits, mark(fits))
    repinned = tested == newest
    compat_ok = fits == "0" and test == "0" and repinned and update in ("", "0")
    if compat_ok:
        close(COMPAT, f"Kid Mode fits and passes its tests on Horizon {name} again ([run]({run})).")
    else:
        test_on = "the newest build" if repinned else f"the pin (`{tested[:7] or '?'}`), since the repin failed"
        body = (
            f"Kid Mode isn't known to work on the newest Horizon build. Filed by the Horizon watch, "
            f"updated on each run, closed when everything passes.\n\n{head}\n"
            f"| Check | Result |\n| --- | --- |\n"
            f"| `cargo horizon-mod fits`: the released mod, unchanged, on the newest build | {fits_word} |\n"
            f"| `cargo horizon-mod update`: repin to the newest build | {mark(update) if behind else '➖ already pinned to it'} |\n"
            f"| Clippy and `cargo test` on {test_on} | {mark(test)} |\n\n"
            f"If none of the checks ran, the watch itself failed before them: see the run.\n\n"
            f"### fits\n\n{block(output('fits.txt'))}\n\n"
            f"### update\n\n{block(output('update.txt', 150) if behind else '')}\n\n"
            f"### Clippy and tests (last 120 lines)\n\n{block(output('test.txt', 120))}\n"
        )
        upsert(
            COMPAT,
            f"Kid Mode on Horizon {date}: compatibility check failed",
            body,
            newest,
            f"Horizon {name} is out and the checks still fail on it; the description is updated ([run]({run})).",
        )

    # 2. Mod API changes that matter to Kid Mode. The API changes almost every Nightly, so the
    # issue is only for changes to items Kid Mode uses, or additions the watchlist asks for.
    old_hash, new_hash = surface_hash(pinned), surface_hash(newest)
    update_text = output("update.txt")
    used_changed = behind and update == "0" and not UNCHANGED.search(update_text)
    hits = watch_hits(output("sdk-diff.txt")) if behind and diff == "0" else []
    if not behind or (old_hash and old_hash == new_hash) or not (used_changed or hits):
        close(API_LABEL, f"Nothing Kid Mode uses or watches for differs between the pin and {name} ([run]({run})).")
        return
    found = "".join(f"- `{item}` ({change}): {feature}\n" for item, change, feature in hits)
    body = (
        f"Horizon's newest build changes something Kid Mode cares about. Filed by the Horizon watch, "
        f"updated on each run, closed once nothing it reports is left (move the pin with "
        f"`cargo horizon-mod update`, or drop a rule from `.github/horizon-watchlist.txt`).\n\n{head}\n"
        f"### On the watchlist\n\n{found or 'Nothing.'}\n\n"
        f"### What a repin changes in what Kid Mode uses\n\n"
        f"From `cargo horizon-mod update --horizon {newest[:7]}` (run in CI only, nothing committed)"
        f"{'' if used_changed else ': nothing it uses changed'}:\n\n"
        f"{block(output('update.txt', 150))}\n\n"
        f"### Everything that changed in the mod API since the pin\n\n"
        f"From `cargo horizon-mod sdk diff {pinned[:7]} {newest[:7]}`:\n\n"
        f"<details><summary>The diff</summary>\n\n{block(output('sdk-diff.txt'))}\n\n</details>\n"
    )
    # Comment only when what's reported changes, not on every new Nightly.
    key = hashlib.sha256(json.dumps([hits, used_changed and update_text]).encode()).hexdigest()[:16]
    upsert(
        API_LABEL,
        f"Horizon mod API: changes that matter to Kid Mode (newest: {date})",
        body,
        key,
        f"Horizon {name} changes what's reported: the description is updated ([run]({run})).",
    )

if __name__ == "__main__":
    main()

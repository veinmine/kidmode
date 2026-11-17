#!/usr/bin/env python3
"""Refresh compatibility.json's `mixin_schema` and `source_sha256`.

Runs the launcher's horizon-mixins `plan` on the pinned Horizon source (the
launcher downloads it on the first `native-build`) and records the plan schema
and the SHA-256 of every Horizon file the mod injects into, as
horizon-native-mod-template does. Run it after changing the hooks or the pin:

    python3 tools/update_compat.py
"""

import glob
import json
import os
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NATIVE = os.path.expanduser("~/.local/share/horizon-launcher/native")


def main() -> int:
    compat_path = os.path.join(ROOT, "compatibility.json")
    with open(compat_path, encoding="utf-8") as f:
        compat = json.load(f)
    source = os.path.join(NATIVE, "source", compat["horizon_revision"])
    if not os.path.isdir(source):
        sys.exit(f"no Horizon source at {source}: run horizon-launcher native-build first")
    tools = sorted(glob.glob(os.path.join(NATIVE, "tools", "*", "bin", "horizon-mixins")),
                   key=os.path.getmtime)
    if not tools:
        sys.exit("no horizon-mixins in the launcher's cache: run horizon-launcher native-build first")

    with tempfile.TemporaryDirectory() as tmp:
        out = os.path.join(tmp, "plan.json")
        subprocess.run([tools[-1], "plan", "--workspace", source,
                        "--mods", os.path.join(ROOT, "mixins"), "--out", out],
                       check=True, stdout=subprocess.DEVNULL)
        with open(out, encoding="utf-8") as f:
            plan = json.load(f)

    files = {i["file"] for i in plan["injections"]} | {m["file"] for m in plan["modules"]}
    compat["mixin_schema"] = plan["schema"]
    compat["source_sha256"] = {path: plan["inputs"][path] for path in sorted(files)}
    with open(compat_path, "w", encoding="utf-8") as f:
        json.dump(compat, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print(f"schema {plan['schema']}, {len(files)} source files")
    return 0


if __name__ == "__main__":
    sys.exit(main())

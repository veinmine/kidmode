// The mod's row in Options > Mods, in the game's language.
fn mord_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::mobs::mord as m;
    use horizon_client::lang;
    let language = lang::current();
    let code = lang::code();
    let tr = |key: &str| {
        m::text(&code, key, |k| language.as_ref().and_then(|l| l.get(k)).map(str::to_owned))
    };
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", tr("mord.name"), m::VERSION),
        description: tr("mord.description"),
        keys: Vec::new(),
    });
}

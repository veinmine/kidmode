// The mod's row in Options > Mods, in the game's language.
fn kidmode_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::mobs::kidmode as m;
    use horizon_client::lang;
    let language = lang::current();
    let code = lang::code();
    let tr = |key: &str| {
        m::text(&code, key, |k| language.as_ref().and_then(|l| l.get(k)).map(str::to_owned))
    };
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", tr("kidmode.name"), m::VERSION),
        description: tr("kidmode.description"),
        keys: Vec::new(),
    });
}

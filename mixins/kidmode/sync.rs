// What the server sends every player's game about Kid Mode's mobs (Horizon's
// synced attachments: NeoForge's `AttachmentType` with `sync`). Kept here, in
// the game crate, because `runtime.rs` is also built on its own for its tests.

use horizon_client::mod_api::{AttachmentCodec, AttachmentError, AttachmentType, Compound, Tag};

/// A mob the mod tamed: who tamed it, when known (a mob tamed with `/tag`, or
/// before 0.9.1, has no owner).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tamed {
    pub owner: Option<u128>,
    pub owner_name: Option<String>,
    /// Their `minecraft:profile` with the skin, as the server had it when they
    /// tamed the mob, for a player head that shows their skin at once.
    pub profile: Option<Tag>,
}

/// On a mob the mod tamed (the `kidmode_tamed` tag, which isn't sent to
/// players): present while it is tamed, so every player's game can show the
/// heart, or the tamer's head, in its taming-hint bubble.
pub static TAMED: AttachmentType<Tamed> = AttachmentType {
    namespace: "kidmode",
    name: "tamed",
    default: Tamed::default,
    codec: Some(AttachmentCodec {
        encode: encode,
        decode: decode,
    }),
    copy_on_death: false,
    sync: true,
};

/// A UUID as Minecraft's four ints, most significant first.
pub fn uuid_ints(uuid: u128) -> Vec<i32> {
    (0..4).map(|i| (uuid >> (96 - 32 * i)) as u32 as i32).collect()
}

pub fn uuid_from_ints(ints: &[i32]) -> Option<u128> {
    (ints.len() == 4).then(|| ints.iter().fold(0u128, |u, &i| u << 32 | u128::from(i as u32)))
}

fn encode(tamed: &Tamed) -> Result<Compound, AttachmentError> {
    let mut data: Compound = [("tamed".into(), Tag::Byte(1))].into();
    if let Some(owner) = tamed.owner {
        data.insert("owner".into(), Tag::IntArray(uuid_ints(owner)));
    }
    if let Some(name) = &tamed.owner_name {
        data.insert("owner_name".into(), Tag::String(name.as_str().into()));
    }
    if let Some(profile) = &tamed.profile {
        data.insert("profile".into(), profile.clone());
    }
    Ok(data)
}

fn decode(data: &Compound) -> Result<Tamed, AttachmentError> {
    Ok(Tamed {
        owner: match data.get(&"owner".into()) {
            Some(Tag::IntArray(ints)) => uuid_from_ints(ints),
            _ => None,
        },
        owner_name: match data.get(&"owner_name".into()) {
            Some(Tag::String(name)) => Some(name.to_string_lossy()),
            _ => None,
        },
        profile: data.get(&"profile".into()).cloned(),
    })
}

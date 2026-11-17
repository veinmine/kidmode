// What the server sends every player's game about Kid Mode's mobs (Horizon's
// synced attachments: NeoForge's `AttachmentType` with `sync`). Kept here, in
// the game crate, because `runtime.rs` is also built on its own for its tests.

use horizon_client::mod_api::{AttachmentCodec, AttachmentError, AttachmentType, Tag};

/// On a mob the mod tamed (the `kidmode_tamed` tag, which isn't sent to
/// players): present while it is tamed, so every player's game can show the
/// heart in its taming-hint bubble.
pub static TAMED: AttachmentType<bool> = AttachmentType {
    namespace: "kidmode",
    name: "tamed",
    default: || false,
    codec: Some(AttachmentCodec {
        encode: |tamed| Ok([("tamed".into(), Tag::Byte(i8::from(*tamed)))].into()),
        decode: |data| match data.get(&"tamed".into()) {
            Some(Tag::Byte(b)) => Ok(*b != 0),
            _ => Err(AttachmentError::Decode),
        },
    }),
    copy_on_death: false,
    sync: true,
};

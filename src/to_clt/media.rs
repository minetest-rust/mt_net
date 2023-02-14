use super::*;

#[mt_derive(to = "clt", repr = "u8", tag = "type")]
pub enum TileAnim {
    None = 0,
    VerticalFrame {
        n_frames: [u16; 2],
        duration: f32,
    },
    SpriteSheet {
        aspect_ratio: [u8; 2],
        duration: f32,
    },
}

#[mt_derive(to = "clt")]
pub struct ItemDef; // TODO

#[mt_derive(to = "clt")]
pub struct NodeDef; // TODO

#[mt_derive(to = "clt", repr = "u8")]
pub enum SoundSrcType {
    Nowhere = 0,
    Pos,
    Obj,
}

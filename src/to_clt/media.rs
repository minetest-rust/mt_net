use super::*;

#[mt_derive(to = "clt")]
pub struct TileAnim; // TODO

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

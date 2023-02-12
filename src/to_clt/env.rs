use super::*;

#[mt_derive(to = "clt")]
pub struct ObjProps {
    #[mt(const8 = 4)] // version
    pub max_hp: u16, // player only
    pub collide_with_nodes: bool,
    pub weight: f32, // deprecated
    pub collision_box: ([f32; 3], [f32; 3]),
    pub selection_box: ([f32; 3], [f32; 3]),
    pub pointable: bool,
    pub visual: String,
    pub visual_size: [f32; 3],
    pub textures: Vec<String>,
    pub sprite_sheet_size: [i16; 2], // in sprites
    pub sprite_pos: [i16; 2],        // in sprite sheet
    pub visible: bool,
    pub make_footstep_sounds: bool,
    pub rotate_speed: f32, // in radians per second
    pub mesh: String,
    pub colors: Vec<Color>,
    pub collide_with_objs: bool,
    pub step_height: f32,
    pub face_rotate_dir: bool,
    pub face_rotate_dir_off: f32, // in degrees
    pub backface_cull: bool,
    pub nametag: String,
    pub nametag_color: Color,
    pub face_rotate_speed: f32, // in degrees per second
    pub infotext: String,
    pub itemstring: String,
    pub glow: i8,
    pub max_breath: u16, // player only
    pub eye_height: f32, // player only
    pub zoom_fov: f32,   // in degrees. player only
    pub use_texture_alpha: bool,
    pub dmg_texture_mod: String, // suffix
    pub shaded: bool,
    pub show_on_minimap: bool,
    pub nametag_bg: Color,
}

#[mt_derive(to = "clt")]
pub struct ObjPos {
    pub pos: [f32; 3],
    pub vel: [f32; 3],
    pub acc: [f32; 3],
    pub rot: [f32; 3],
    pub interpolate: bool,
    pub end: bool,
    pub update_interval: f32,
}

#[mt_derive(to = "clt")]
pub struct ObjSprite {
    pub frame0: [i16; 2],
    pub frames: u16,
    pub frame_duration: f32,
    pub view_angle_frames: bool,
}

#[mt_derive(to = "clt")]
pub struct ObjAnim {
    pub frames: [i32; 2],
    pub speed: f32,
    pub blend: f32,
    pub no_loop: bool,
}

#[mt_derive(to = "clt")]
pub struct ObjBonePos {
    pub pos: [f32; 3],
    pub rot: [f32; 3],
}

#[mt_derive(to = "clt")]
pub struct ObjAttach {
    pub parent_id: u16,
    pub bone: String,
    pub pos: [f32; 3],
    pub rot: [f32; 3],
    pub force_visible: bool,
}

#[mt_derive(to = "clt")]
pub struct ObjPhysicsOverride {
    pub walk: f32,
    pub jump: f32,
    pub gravity: f32,
    // the following are player only
    pub no_sneak: bool,
    pub no_sneak_glitch: bool,
    pub old_sneak: bool,
}

#[mt_derive(to = "clt", repr = "u8", tag = "type", content = "data")]
pub enum ObjMsg {
    Props(ObjProps) = 0,
    Pos(ObjPos),
    TextureMod {
        #[serde(rename = "mod")]
        texture_mod: String,
    },
    Sprite(ObjSprite),
    Hp {
        hp: u16,
    },
    ArmorGroups {
        armor: HashMap<String, u16>,
    },
    Anim(ObjAnim),
    BonePos {
        bone: String,
        pos: ObjBonePos,
    },
    Attach(ObjAttach),
    PhysicsOverride(ObjPhysicsOverride),
    SpawnInfant {
        id: u16,
        #[mt(const8 = 101)] // GenericCAO
        infant_type: (),
    } = 11,
    AnimSpeed {
        speed: f32,
    },
}

#[mt_derive(to = "clt")]
pub struct ObjIdMsg {
    pub id: u16,
    #[mt(size16)]
    pub msg: ObjMsg,
}

#[mt_derive(to = "clt")]
pub struct ObjInitMsg(#[mt(size32)] ObjMsg);

#[mt_derive(to = "clt")]
pub struct ObjInitData {
    #[mt(const8 = 1)] // version
    pub name: String,
    pub is_player: bool,
    pub id: u16,
    pub pos: [f32; 3],
    pub rot: [f32; 3],
    pub hp: u16,
    #[mt(len8)]
    pub msgs: Vec<ObjInitMsg>,
}

#[mt_derive(to = "clt")]
pub struct ObjAdd {
    pub id: u16,
    #[mt(const8 = 101)] // GenericCAO
    #[mt(size32)]
    pub init_data: ObjInitData,
}

#[mt_derive(to = "clt", repr = "u8", enumset)]
pub enum MapBlockFlag {
    IsUnderground = 0,
    DayNightDiff,
    LightExpired,
    NotGenerated,
}

pub const ALWAYS_LIT_FROM: u16 = 0xf000;

#[mt_derive(to = "clt")]
pub struct MapBlock {
    pub flags: EnumSet<MapBlockFlag>,
    pub lit_from: u16,

    #[mt(const8 = 2)]
    #[serde(skip)]
    pub param0_size: (),

    #[mt(const8 = 2)]
    #[serde(skip)]
    pub param12_size: (),

    #[serde(with = "serde_arrays")]
    pub param_0: [u16; 4096],
    #[serde(with = "serde_arrays")]
    pub param_1: [u8; 4096],
    #[serde(with = "serde_arrays")]
    pub param_2: [u8; 4096],

    pub node_metas: HashMap<u16, NodeMeta>,

    #[mt(const8 = 2)]
    #[serde(skip)]
    pub version: (),
}

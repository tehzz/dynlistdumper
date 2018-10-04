use std::{fmt};
/// Standard object types as the same bitflags as the game.
bitflags!{
    pub struct ObjFlag: u32 {
        const GROUPS    = 0x00000001;
        const BONES     = 0x00000002;
        const JOINTS    = 0x00000004;
        const PARTICLES = 0x00000008;
        const SHAPES    = 0x00000010;
        const NETS      = 0x00000020;
        const PLANES    = 0x00000040;
        const FACES     = 0x00000080;
        const VERTICES  = 0x00000100;
        const CAMERAS   = 0x00000200;
        const MATERIALS = 0x00000800;
        const WEIGHTS   = 0x00001000;
        const GADGETS   = 0x00002000;
        const VIEWS     = 0x00004000;
        const LABELS    = 0x00008000;
        const ANIMATORS = 0x00010000;
        const VALPTRS   = 0x00020000;
        const LIGHTS    = 0x00080000;
        const ZONES     = 0x00100000;
        const UNK200000 = 0x00200000;
    }
}

impl fmt::Display for ObjFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comma = format!("{:?}", self).replace(" | ", ", ").to_ascii_lowercase();
        write!(f, "{}", comma)
    }
}
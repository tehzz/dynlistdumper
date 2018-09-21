/// All DynList commands as determined from function [Name; OFFSET] in SM64 J (GAME ID)
#[derive(Debug)]
pub enum DynCmd {
    Start,
    Stop,
    Known(u32),
    Unk(u32),
}

impl DynCmd {
    pub fn from_struct(cmd: &[u32; 6]) -> Self {
        use self::DynCmd::*;
        match cmd[0] {
            0xD1D4 => Start,
            58 => Stop,
            k @ 0...58 => Known(k),
            u @ _ => Unk(u),
        }
    }
}
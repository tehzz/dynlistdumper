use std::fmt;
use dynlist::dobj_types::DObjType;

/// This is used by the game as a pointer, so be able to indicate it
#[derive(Debug)]
pub struct Ptr(u32);
impl fmt::Display for Ptr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ptr<{:#010X}>", self.0)
    }
}

// Maybe want to add something to check if this is char * vs an int?
/// The weird maybe-int, maybe-char* id type. 
#[derive(Debug)]
pub struct DynId(u32);
impl fmt::Display for DynId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID<{:#X}>", self.0)
    }
}


/// All DynList commands as determined from function [Name; OFFSET] in SM64 J (GAME ID)
#[derive(Debug)]
pub enum DynCmd {
    Start,
    Stop,
    Jump(Ptr),  //maybe should be jump...? the function is recursive...
    MakeObj(DObjType, DynId),
    StartGroup(DynId),
    EndGroup(DynId),
    Known(u32),
    Unk(u32),
}

impl DynCmd {
    pub fn from_struct(cmd: &[u32; 6]) -> Self {
        use self::DynCmd::*;
        match cmd[0] {
            0xD1D4 => Start,
            58 => Stop,
            12 => Jump(Ptr(cmd[1])),
            15 => MakeObj(cmd[2].into(), DynId(cmd[1])),
            16 => StartGroup(DynId(cmd[1])),
            17 => EndGroup(DynId(cmd[1])),
            k @ 0...58 => Known(k),
            u @ _ => Unk(u),
        }
    }
}
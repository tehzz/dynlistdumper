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

/// How the arguments are used
#[derive(Debug)]
pub enum DynArg {
    Void,
    First,
    Second, 
    Both,
    SwapBoth,
    VecXYZ,
    Vector,
}

/// Printing info for all commands
#[derive(Debug)]
pub struct CmdInfo {
    pub base: &'static str,
    pub desc: &'static str,
    pub kind: DynArg,
    pub id: u32,
}

/// All DynList commands as determined from function [Name; OFFSET] in SM64 J (GAME ID)
#[derive(Debug)]
pub enum DynCmd {
    Start,
    Stop,
    UseIntId(bool),
    Jump(Ptr),
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
            0  => UseIntId(cmd[2] != 0),
            12 => Jump(Ptr(cmd[1])),
            15 => MakeObj(cmd[2].into(), DynId(cmd[1])),
            16 => StartGroup(DynId(cmd[1])),
            17 => EndGroup(DynId(cmd[1])),
            k @ 0...58 => Known(k),
            u @ _ => Unk(u),
        }
    }
    /// Create an iterator over the real variants of the DynCmd enum
    pub fn variants() -> impl Iterator<Item=CmdInfo> {
        use self::DynCmd::*;
        [ 
            Start, Stop,
            UseIntId(false), 
        ].into_iter()
        .map(|c| c.info())
    }
    fn info(&self) -> CmdInfo {
        use self::DynCmd::*;
        use self::DynArg::*;
        match self {
            Start => CmdInfo {
                base: "StartList",
                desc: "Necessary start command for the dynlist. List will not process otherwise.",
                kind: Void,
                id: 0xD1D4,
            },
            Stop => CmdInfo {
                base: "StopList",
                desc: "Necessary stop command for the dynlist.",
                kind: Void,
                id: 58,
            },
            UseIntId(..) => CmdInfo {
                base: "UseIntId",
                desc: "Subsequent dynobj ids should be treated as ints, not as c-string pointers.",
                kind: Second,
                id: 0,
            },
            Jump(..) => CmdInfo {
                base: "JumpToList",
                desc: "Jump to pointed dynlist. Once that list has finished processing, flow returns to current list.",
                kind: First,
                id: 12,
            },
            MakeObj(..) => CmdInfo {
                base: "MakeDynObj",
                desc: "Make an object of the specified type and id, and add that object to the dynobj pool.",
                kind: SwapBoth,
                id: 15,
            },
            StartGroup(..) => CmdInfo {
                base: "StartGroup",
                desc: "Make a group object that will contain all subsequently created objects once the EndGroup command is called.",
                kind: First,
                id: 16,
            },
            EndGroup(..) => CmdInfo {
                base: "EndGroup",
                desc: "Collect all objects created after the StartGroup command.",
                kind: First,
                id: 17,
            },
            Known(id) => CmdInfo {
                base: "Known",
                desc: "Filler until all commands are known.",
                kind: First,
                id: *id,
            },
            Unk(id) => CmdInfo {
                base: "Unknown",
                desc: "N/A",
                kind: Void,
                id: *id,
            },
        }
    }
}

impl fmt::Display for DynCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DynCmd::*;
        let info = self.info();
        match self {
            Start => void_macro(f, info.base),
            Stop => void_macro(f, info.base),
            UseIntId(b) => one_param(f, info.base, if *b {"TRUE"} else {"FALSE"}),
            Jump(dl) => one_param(f, info.base, dl),
            MakeObj(t, id) => two_param(f, info.base, t, id),
            StartGroup(id) => one_param(f, info.base, id),
            EndGroup(id) => one_param(f, info.base, id),
            Known(cmd) => write!(f, "Known cmd <{}>", cmd),
            Unk(val) => write!(f, "Unknown cmd <{}>", val),
        }
    }
}

/* Helper function to write a command as a macro */
#[inline]
fn void_macro(f: &mut fmt::Formatter, name: &str) -> fmt::Result {
    write!(f, "{}", name)
}
#[inline]
fn one_param<D: fmt::Display> (f: &mut fmt::Formatter, name: &str, param: D) -> fmt::Result {
    write!(f, "{} {}", name, param)
}

#[inline]
fn two_param<D, E> (f: &mut fmt::Formatter, name: &str, p1: D, p2: E) -> fmt::Result 
where D: fmt::Display,
      E: fmt::Display
{
    write!(f, "{} {}, {}", name, p1, p2)
}
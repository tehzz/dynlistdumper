use std::fmt;
use dynlist::dobj_types::DObjType;
use dynlist::objs;

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

#[derive(Debug, Default, Copy, Clone)]
pub struct Vector{ x: f32, y: f32, z: f32 }
impl Vector {
    const ZERO: Vector = Vector{x: 0.0, y: 0.0, z: 0.0};
}
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec<{},{},{}>", self.x, self.y, self.z)
    }
}
impl<'a> From<&'a [u32]> for Vector {
    fn from(arr: &[u32]) -> Self {
        assert!(arr.len() >= 3);
        let x = f32::from_bits(arr[0]);
        let y = f32::from_bits(arr[1]);
        let z = f32::from_bits(arr[2]);
        Vector{x, y, z}
    }
}

/// How the arguments are used
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    pub objs: objs::ObjFlag,    // which, if any, object types this command can act on
    pub id: u32,
}

/// All DynList commands as determined from function [Name; OFFSET] in SM64 J (GAME ID)
#[derive(Debug)]
pub enum DynCmd {
    Start,
    Stop,
    UseIntId(bool),
    SetInitPos(Vector),
    SetRelPos(Vector),
    SetWorldPos(Vector),
    SetNormal(Vector),
    SetScale(Vector),
    SetRotation(Vector),
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
            1  => SetInitPos(cmd[3..6].into()),
            2  => SetRelPos(cmd[3..6].into()),
            3  => SetWorldPos(cmd[3..6].into()),
            4  => SetNormal(cmd[3..6].into()),
            5  => SetScale(cmd[3..6].into()),
            6  => SetRotation(cmd[3..6].into()),
            12 => Jump(Ptr(cmd[1])),
            15 => MakeObj(cmd[2].into(), DynId(cmd[1])),
            16 => StartGroup(DynId(cmd[1])),
            17 => EndGroup(DynId(cmd[1])),
            k @ 0...58 => Known(k),
            u @ _ => Unk(u),
        }
    }
    /// Create an iterator over the real/necessary variants of the DynCmd enum
    pub fn variants() -> impl Iterator<Item=CmdInfo> {
        use self::DynCmd::*;
        [ 
            Start, Stop,
            UseIntId(false), 
            SetInitPos(Vector::ZERO),
            SetRelPos(Vector::ZERO),
            SetWorldPos(Vector::ZERO),
            SetNormal(Vector::ZERO),
            SetScale(Vector::ZERO),
            SetRotation(Vector::ZERO),
        ].into_iter()
        .map(|c| c.info())
    }
    /// Basic info for a command
    fn info(&self) -> CmdInfo {
        use self::DynCmd::*;
        use self::DynArg::*;
        use self::objs::ObjFlag as O;
        match self {
            Start => CmdInfo {
                base: "StartList",
                desc: "Necessary start command for the dynlist. List will not process otherwise.",
                kind: Void,
                objs: O::empty(),
                id: 0xD1D4,
            },
            Stop => CmdInfo {
                base: "StopList",
                desc: "Necessary stop command for the dynlist.",
                kind: Void,
                objs: O::empty(),
                id: 58,
            },
            UseIntId(..) => CmdInfo {
                base: "UseIntId",
                desc: "Subsequent dynobj ids should be treated as ints, not as C string pointers.",
                kind: Second,
                objs: O::empty(),
                id: 0,
            },
            SetInitPos(..) => CmdInfo {
                base: "SetInitialPosition",
                desc: "Set the initial position of the current object",
                kind: VecXYZ,
                objs: O::JOINTS | O::NETS | O::PARTICLES | O::CAMERAS | O::VERTICES,
                id: 1,
            },
            SetRelPos(..) => CmdInfo {
                base: "SetRelativePosition",
                desc: "Set the relative position of the current object",
                kind: VecXYZ,
                objs: O::JOINTS | O::LABELS | O::PARTICLES | O::CAMERAS | O::VERTICES,
                id: 2,
            },
            SetWorldPos(..) => CmdInfo {
                base: "SetWorldPosition",
                desc: "Set the world position of the current object",
                kind: VecXYZ,
                objs: O::JOINTS | O::NETS | O::GADGETS | O::VIEWS | O::CAMERAS | O::VERTICES,
                id: 3,
            },
            SetNormal(..) => CmdInfo {
                base: "SetNormal",
                desc: "Set the normal of the current object",
                kind: VecXYZ,
                objs: O::VERTICES,
                id: 4,
            },
            SetScale(..) => CmdInfo {
                base: "SetScale",
                desc: "Set the scale of the current object",
                kind: VecXYZ,
                objs: O::JOINTS | O::NETS | O::VIEWS | O::PARTICLES | O::GADGETS | O::LIGHTS,
                id: 5,
            },
            SetRotation(..) => CmdInfo {
                base: "SetRotation",
                desc: "Set the rotation of the current object",
                kind: VecXYZ,
                objs: O::JOINTS | O::NETS,
                id: 6,
            },
            Jump(..) => CmdInfo {
                base: "JumpToList",
                desc: "Jump to pointed dynlist. Once that list has finished processing, flow returns to current list.",
                kind: First,
                objs: O::empty(),
                id: 12,
            },
            MakeObj(..) => CmdInfo {
                base: "MakeDynObj",
                desc: "Make an object of the specified type and id, and add that object to the dynobj pool.",
                kind: SwapBoth,
                objs: O::empty(),
                id: 15,
            },
            StartGroup(..) => CmdInfo {
                base: "StartGroup",
                desc: "Make a group object that will contain all subsequently created objects once the EndGroup command is called.",
                kind: First,
                objs: O::empty(),
                id: 16,
            },
            EndGroup(..) => CmdInfo {
                base: "EndGroup",
                desc: "Collect all objects created after the StartGroup command.",
                kind: First,
                objs: O::GROUPS,
                id: 17,
            },
            Known(id) => CmdInfo {
                base: "Known",
                desc: "Filler until all commands are known.",
                kind: First,
                objs: O::empty(),
                id: *id,
            },
            Unk(id) => CmdInfo {
                base: "Unknown",
                desc: "N/A",
                kind: Void,
                objs: O::empty(),                
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
            SetInitPos(vec) => full_vec(f, info.base, vec),
            SetRelPos(vec) => full_vec(f, info.base, vec),
            SetWorldPos(vec) => full_vec(f, info.base, vec),
            SetNormal(vec) => full_vec(f, info.base, vec),
            SetScale(vec) => full_vec(f, info.base, vec),
            SetRotation(vec) => full_vec(f, info.base, vec),
            Jump(dl) => one_param(f, info.base, dl),
            MakeObj(t, id) => two_param(f, info.base, t, id),
            StartGroup(id) => one_param(f, info.base, id),
            EndGroup(id) => one_param(f, info.base, id),
            Known(cmd) => write!(f, "Known cmd <{}>", cmd),
            Unk(val) => write!(f, "Unknown cmd <{}>", val),
        }
    }
}

/* Helper functions to write a command as a macro */
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
    where D: fmt::Display, E: fmt::Display
{
    write!(f, "{} {}, {}", name, p1, p2)
}
#[inline]
fn full_vec(f: &mut fmt::Formatter, name: &str, vec: &Vector) -> fmt::Result {
    write!(f, "{} {:?}, {:?}, {:?}", name, vec.x, vec.y, vec.z)
}

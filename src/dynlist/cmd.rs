use std::fmt;
use dynlist::dobj_types::DObjType;
use dynlist::param_ptr::PtrParam;
use dynlist::objs;

/// This is used by the game as a pointer, so be able to indicate it
#[derive(Debug)]
pub struct Ptr(u32);
impl Ptr {
    const NULL: Ptr = Ptr(0);
}
impl fmt::Display for Ptr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 != 0 {
            write!(f, "Ptr<{:#010X}>", self.0)
        } else {
            write!(f, "NULL")
        }
    }
}

// Maybe want to add something to check if this is char * vs an int?
/// The weird maybe-int, maybe-char* id type. 
#[derive(Debug)]
pub struct DynId(u32);
impl DynId {
    const NULL: DynId = DynId(0);
}
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
    VecX,
    VecPtr,
    SecVecX,
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
    SetHeaderFlag(u32),
    SetFlag(u32),
    ClearFlag(u32),
    SetFriction(Vector),
    SetSpring(f32),
    SetColourNum(u32),  // TODO: ennumerate?
    Jump(Ptr),
    MakeObj(DObjType, DynId),
    StartGroup(DynId),
    EndGroup(DynId),
    AddToGroup(DynId),
    SetType(u32),
    SetMtlGroup(DynId),
    SetNodeGroup(DynId),
    SetSkinShape(DynId),
    SetPlaneGroup(DynId),
    SetShpPtrPtr(Ptr),
    SetShpPtr(DynId),
    SetShpOff(Vector),
    SetCoG(Vector),
    LinkWith(DynId),
    LinkWithPtr(Ptr),
    UseObj(DynId),
    SetCtrlType(u32),
    SetSkinWgt(u32, f32),
    SetAmbient(Vector),
    SetDiffuse(Vector),
    SetId(u32),
    SetMtl(Ptr, u32),
    MapMtls(DynId),
    MapVtx(DynId),
    Attach(DynId),
    AttachTo(u32, DynId),
    SetAttOff(Vector),
    CpyStrId(Ptr),
    ParamF(u32, f32),
    ParamPtr(PtrParam, Ptr),
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
            7  => SetHeaderFlag(cmd[2]),
            8  => SetFlag(cmd[2]),
            9  => ClearFlag(cmd[2]),
            10 => SetFriction(cmd[3..6].into()),
            11 => SetSpring(f32::from_bits(cmd[3])),
            12 => Jump(Ptr(cmd[1])),
            13 => SetColourNum(cmd[2]),
            // missing 14
            15 => MakeObj(cmd[2].into(), DynId(cmd[1])),
            16 => StartGroup(DynId(cmd[1])),
            17 => EndGroup(DynId(cmd[1])),
            18 => AddToGroup(DynId(cmd[1])),
            19 => SetType(cmd[2]),
            20 => SetMtlGroup(DynId(cmd[1])),
            21 => SetNodeGroup(DynId(cmd[1])),
            22 => SetSkinShape(DynId(cmd[1])),
            23 => SetPlaneGroup(DynId(cmd[1])),
            24 => SetShpPtrPtr(Ptr(cmd[1])),
            25 => SetShpPtr(DynId(cmd[1])),
            26 => SetShpOff(cmd[3..6].into()),
            27 => SetCoG(cmd[3..6].into()),
            28 => LinkWith(DynId(cmd[1])),
            29 => LinkWithPtr(Ptr(cmd[1])),
            30 => UseObj(DynId(cmd[1])),
            31 => SetCtrlType(cmd[2]),
            32 => SetSkinWgt(cmd[2], f32::from_bits(cmd[3])),
            33 => SetAmbient(cmd[3..6].into()),
            34 => SetDiffuse(cmd[3..6].into()),
            35 => SetId(cmd[2]),
            36 => SetMtl(Ptr(cmd[1]), cmd[2]),
            37 => MapMtls(DynId(cmd[1])),
            38 => MapVtx(DynId(cmd[1])),
            39 => Attach(DynId(cmd[1])),
            40 => AttachTo(cmd[2], DynId(cmd[1])),
            41 => SetAttOff(cmd[3..6].into()),
            // missing 42
            43 => CpyStrId(Ptr(cmd[1])),
            44 => ParamF(cmd[2], f32::from_bits(cmd[3])),
            45 => ParamPtr(cmd[2].into(), Ptr(cmd[1])),

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
            SetHeaderFlag(0),
            SetFlag(0),
            ClearFlag(0),
            SetFriction(Vector::ZERO),
            SetSpring(0.0),
            Jump(Ptr::NULL),
            SetColourNum(0),
            MakeObj(DObjType::D_NET, DynId::NULL),
            StartGroup(DynId::NULL),
            EndGroup(DynId::NULL),
            AddToGroup(DynId::NULL),
            SetType(0),
            SetMtlGroup(DynId:: NULL),
            SetNodeGroup(DynId:: NULL),
            SetSkinShape(DynId:: NULL),
            SetPlaneGroup(DynId:: NULL),
            SetShpPtrPtr(Ptr::NULL),
            SetShpPtr(DynId:: NULL),
            SetShpOff(Vector::ZERO),
            SetCoG(Vector::ZERO),
            LinkWith(DynId::NULL),
            LinkWithPtr(Ptr::NULL),
            UseObj(DynId::NULL),
            SetCtrlType(0),
            SetSkinWgt(0, 0.0),
            SetAmbient(Vector::ZERO),
            SetDiffuse(Vector::ZERO),
            SetId(0),
            SetMtl(Ptr::NULL, 0),
            MapMtls(DynId::NULL),
            MapVtx(DynId::NULL),
            Attach(DynId::NULL),
            AttachTo(0, DynId::NULL),
            SetAttOff(Vector::ZERO),
            CpyStrId(Ptr::NULL),
            ParamF(0, 0.0),
            ParamPtr(PtrParam::PARAM_OBJ_VTX,Ptr::NULL),
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
            SetHeaderFlag(..) => CmdInfo {
                base: "SetHeaderFlag",
                desc: "Set the half-word flag in the header of the current dynobj",
                kind: Second,
                objs: O::all(),
                id: 7,
            },
            SetFlag(..) => CmdInfo {
                base: "SetFlag",
                desc: "Set the bits in an object specific flag with the provided flag",
                kind: Second,
                objs: O::JOINTS | O::BONES | O::NETS | O::CAMERAS | O::VIEWS | O::SHAPES | O::PARTICLES | O::LIGHTS,
                id: 8,
            },
            ClearFlag(..) => CmdInfo {
                base: "ClearFlag",
                desc: "Clear the bits in an object specific flag with the provided flag",
                kind: Second,
                objs: O::JOINTS | O::BONES | O::NETS | O::CAMERAS | O::PARTICLES,
                id: 9,
            },
            SetFriction(..) => CmdInfo {
                base: "SetFriction",
                desc: "Set the friction vector of a Joint",
                kind: VecXYZ,
                objs: O::JOINTS,
                id: 10,
            },
            SetSpring(..) => CmdInfo {
                base: "SetSpring",
                desc: "Set the spring float of a Bone",
                kind: VecX,
                objs: O::BONES,
                id: 11,
            },
            Jump(..) => CmdInfo {
                base: "JumpToList",
                desc: "Jump to pointed dynlist. Once that list has finished processing, flow returns to current list.",
                kind: First,
                objs: O::empty(),
                id: 12,
            },
            SetColourNum(..) => CmdInfo {
                base: "SetColourNum",
                desc: "Store either the enumerated \"colour\" number in an object, or the RGB float values the number refers to",
                kind: Second,
                objs: O::JOINTS | O::PARTICLES | O::NETS | O::GADGETS | O::FACES,
                id: 13,
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
                desc: "Collect all objects created after the StartGroup command with the same id.",
                kind: First,
                objs: O::GROUPS,
                id: 17,
            },
            AddToGroup(..) => CmdInfo {
                base: "AddToGroup",
                desc: "Add the current dyn object to the Group with the called ID",
                kind: First,
                objs: O::GROUPS,
                id: 18,
            },
            SetType(..) => CmdInfo {
                base: "SetType",
                desc: "Set an object specific type flag.",
                kind: Second,
                objs: O::NETS | O::GADGETS | O::GROUPS | O::JOINTS | O::PARTICLES | O::MATERIALS,
                id: 19,
            },
            SetMtlGroup(..) => CmdInfo {
                base: "SetMaterialGroup",
                desc: "Assign the material Group ID to the current dynobj Shape and check the Shape",
                kind: First,
                objs: O::SHAPES,
                id: 20,
            },
            SetNodeGroup(..) => CmdInfo {
                base: "SetNodeGroup",
                desc: "Attach Group ID to the current dynobj",
                kind: First,
                objs: O::NETS | O::SHAPES | O::GADGETS | O::ANIMATORS,
                id: 21,
            },
            SetSkinShape(..) => CmdInfo {
                base: "SetSkinShape",
                desc: "Set the skin group of the current Net dynobj with the vertices from Shape ID",
                kind: First,
                objs: O::NETS,
                id: 22,
            },
            SetPlaneGroup(..) => CmdInfo {
                base: "SetPlaneGroup",
                desc: "Set the plane group ID of the current dynobj",
                kind: First,
                objs: O::NETS | O::SHAPES,
                id: 23,
            },
            SetShpPtrPtr(..) => CmdInfo {
                base: "SetShapePtrPtr",
                desc: "Set the current dynobj's shape pointer by dereferencing the ptr ptr",
                kind: First,
                objs: O::JOINTS | O::NETS | O::BONES | O::GADGETS | O::PARTICLES | O::LIGHTS,
                id: 24,
            },
            SetShpPtr(..) => CmdInfo {
                base: "SetShapePtr",
                desc: "Set the current dynobj's shape pointer to Shape ID",
                kind: First,
                objs: O::JOINTS | O::NETS | O::BONES | O::GADGETS | O::PARTICLES,
                id: 25,
            },
            SetShpOff(..) => CmdInfo {
                base: "SetShapeOffset",
                desc: "Set offset of the connected shape",
                kind: VecXYZ,
                objs: O::JOINTS,
                id: 26,
            },
            SetCoG(..) => CmdInfo {
                base: "SetCenterOfGravity",
                desc: "Set the center of gravity of the current Net object",
                kind: VecXYZ,
                objs: O::NETS,
                id: 27,
            },
            LinkWith(..) => CmdInfo {
                base: "LinkWith",
                desc: "Link Object ID to the current dynobj",
                kind: First,
                objs: O::CAMERAS | O::GROUPS | O::BONES | O::VIEWS | O::FACES | O::ANIMATORS | O::LABELS,
                id: 28,
            },
            LinkWithPtr(..) => CmdInfo {
                base: "LinkWithPtr",
                desc: "Link Object pointer to the current dynobj",
                kind: First,
                objs: O::CAMERAS | O::GROUPS | O::BONES | O::VIEWS | O::FACES | O::ANIMATORS | O::LABELS,
                id: 29,
            },
            UseObj(..) => CmdInfo {
                base: "UseObj",
                desc: "Set Object ID as the current dynobj",
                kind: First,
                objs: O::all(),
                id: 30,
            },
            SetCtrlType(..) => CmdInfo {
                base: "SetControlType",
                desc: "Set the current Net object's control type field",
                kind: Second,
                objs: O::NETS,
                id: 31,
            },
            SetSkinWgt(..) => CmdInfo {
                base: "SetSkinWeight",
                desc: "Set the weight of the current Joint object with ID and VALUE",
                kind: SecVecX,
                objs: O::JOINTS,
                id: 32,
            },
            SetAmbient(..) => CmdInfo {
                base: "SetAmbient",
                desc: "Set the ambient color of the current Material object",
                kind: VecXYZ,
                objs: O::MATERIALS,
                id: 33,
            },
            SetDiffuse(..) => CmdInfo {
                base: "SetDiffuse",
                desc: "Set the diffuse color of the current Material or Light object",
                kind: VecXYZ,
                objs: O::MATERIALS | O::LIGHTS,
                id: 34,
            },
            SetId(..) => CmdInfo {
                base: "SetId",
                desc: "Set the numerical Object ID field (not dynobj id)",
                kind: Second,
                objs: O::MATERIALS | O::LIGHTS | O::VERTICES | O::JOINTS,
                id: 35,
            },
            SetMtl(..) => CmdInfo {
                base: "SetMaterial",
                desc: "Set the material id of the current Face",
                kind: Both,
                objs: O::FACES,
                id: 36,
            },
            MapMtls(..) => CmdInfo {
                base: "MapMaterials",
                desc: "Map Materials from Group ID to the current Group obj",
                kind: First,
                objs: O::GROUPS,
                id: 37,
            },
            MapVtx(..) => CmdInfo {
                base: "MapVertices",
                desc: "Map vertices from Group ID to the current Group obj",
                kind: First,
                objs: O::GROUPS,
                id: 38,
            },
            Attach(..) => CmdInfo {
                base: "Attach",
                desc: "Stub command",
                kind: First,
                objs: O::JOINTS,
                id: 39,
            },
            AttachTo(..) => CmdInfo {
                base: "AttachTo",
                desc: "Attach the current dynobj with modifications made by FLAG to Object ID",
                kind: SwapBoth,
                objs: O::JOINTS | O::NETS | O::PARTICLES | O::ANIMATORS,
                id: 40,
            },
            SetAttOff(..) => CmdInfo {
                base: "SetAttachOffset",
                desc: "Set the offset of the attached object for the current dynobj",
                kind: VecPtr,
                objs: O::JOINTS | O::NETS | O::PARTICLES,
                id: 41,
            }, 
            CpyStrId(..) => CmdInfo {
                base: "CopyStrToIdBuf",
                desc: "Copy the C-string pointed to by PTR to the dynobj id buf",
                kind: First,
                objs: O::empty(),
                id: 43,
            }, 
            ParamF(..) => CmdInfo {
                base: "SetParamF",
                desc: "Set float paramter PARAM to VALUE. TODO: Ennumerate parameters",
                kind: SecVecX,
                objs: O::SHAPES | O::GADGETS | O::VERTICES,
                id: 44,
            }, 
            ParamPtr(..) => CmdInfo {
                base: "SetParamPtr",
                desc: "Set pointer paramter PARAM to PTR",
                kind: SwapBoth,
                objs: O::LABELS | O::VIEWS | O::FACES,
                id: 45,
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
        let n = info.base;
        match self {
            Start => void_macro(f, n),
            Stop => void_macro(f, n),
            UseIntId(b) => one_param(f, n, if *b {"TRUE"} else {"FALSE"}),
            SetInitPos(vec) => full_vec(f, n, vec),
            SetRelPos(vec) => full_vec(f, n, vec),
            SetWorldPos(vec) => full_vec(f, n, vec),
            SetNormal(vec) => full_vec(f, n, vec),
            SetScale(vec) => full_vec(f, n, vec),
            SetRotation(vec) => full_vec(f, n, vec),
            SetHeaderFlag(flag) => one_param_hex(f, n, flag),
            SetFlag(flag) => one_param_hex(f, n, flag),
            ClearFlag(flag) => one_param_hex(f, n, flag),
            SetFriction(vec) => full_vec(f, n, vec),
            SetSpring(spring) => one_param(f, n, spring),   // might have to make a one_param_d() for the float debug...
            Jump(dl) => one_param(f, n, dl),
            SetColourNum(num) => one_param(f, n, num),
            MakeObj(t, id) => two_param(f, n, t, id),
            StartGroup(id) => one_param(f, n, id),
            EndGroup(id) => one_param(f, n, id),
            AddToGroup(id) => one_param(f, n, id),
            SetType(flag) => one_param(f, n, flag),
            SetMtlGroup(id) => one_param(f, n, id),
            SetNodeGroup(id) => one_param(f, n, id),
            SetSkinShape(id) => one_param(f, n, id),
            SetPlaneGroup(id) => one_param(f, n, id),
            SetShpPtrPtr(dblptr) => one_param(f, n, dblptr),
            SetShpPtr(id) => one_param(f, n, id),
            SetShpOff(vec) => full_vec(f, n, vec),
            SetCoG(vec) => full_vec(f, n, vec),
            LinkWith(id) => one_param(f, n, id),
            LinkWithPtr(ptr) => one_param(f, n, ptr),
            UseObj(id) => one_param(f, n, id),
            SetCtrlType(ctrl) => one_param(f, n, ctrl),
            SetSkinWgt(id, val) => one_and_one(f, n, id, val),
            SetAmbient(rbg) => full_vec(f, n, rbg),
            SetDiffuse(rbg) => full_vec(f, n, rbg),
            SetId(id) => one_param(f, n, id),
            SetMtl(unused, id) => two_param(f, n, unused, id),
            MapMtls(id) => one_param(f, n, id),
            MapVtx(id) => one_param(f, n, id),
            Attach(id) => one_param(f, n, id),
            AttachTo(flag, id) => hex_and_int(f, n, flag, id),
            SetAttOff(vec) => full_vec(f, n, vec),
            CpyStrId(ptr) => one_param(f, n, ptr),
            ParamF(param, fl) => one_and_one(f, n, param, fl),
            ParamPtr(param, ptr) => two_param(f, n, param, ptr),
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
fn one_param_hex<D> (f: &mut fmt::Formatter, name: &str, param: D) -> fmt::Result 
    where D: fmt::Display + fmt::LowerHex
{
    write!(f, "{} {:#x}", name, param)
}
#[inline]
fn one_and_one<D, E> (f: &mut fmt::Formatter, name: &str, p1: D, fl: E) -> fmt::Result 
    where D: fmt::Display, E: fmt::Debug
{
    write!(f, "{} {}, {:?}", name, p1, fl)
}
#[inline]
fn two_param<D, E> (f: &mut fmt::Formatter, name: &str, p1: D, p2: E) -> fmt::Result 
    where D: fmt::Display, E: fmt::Display
{
    write!(f, "{} {}, {}", name, p1, p2)
}
#[inline]
fn hex_and_int<D, E> (f: &mut fmt::Formatter, name: &str, p1: D, p2: E) -> fmt::Result 
    where D: fmt::Display + fmt::LowerHex, E: fmt::Display
{
    write!(f, "{} {:#x}, {}", name, p1, p2)
}
#[inline]
fn full_vec(f: &mut fmt::Formatter, name: &str, vec: &Vector) -> fmt::Result {
    write!(f, "{} {:?}, {:?}, {:?}", name, vec.x, vec.y, vec.z)
}

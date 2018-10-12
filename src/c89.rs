use std::io::{self, Write};
use dynlist::{DynCmd, CmdInfo, DynArg, DObjType, PtrParam};

macro_rules! cmd_type_name {
    () => ( "DynListCmd" )
}
macro_rules! header_name {
    () => ( "_DYN_LIST_MACROS_H_")
}
pub const PREFIX: &'static str = "";
const IFGUARD_START: &'static str = concat!("#ifndef ", header_name![], "\n#define ", header_name![]);
const IFGUARD_END: &'static str = concat!("#endif /* ", header_name![], " */");
const STRUCT_DEC: &'static str = concat!("struct ", cmd_type_name!(), r#" {
    int cmd;
    union { void *ptr; int word; } w1;
    union { void *ptr; int word; } w2;
    float vec[3];
};"#);

pub fn write_header<W: Write>(mut w: W) -> Result<(), io::Error> {
    writeln!(w, "{}\n", IFGUARD_START)?;
    writeln!(w, "{}\n", STRUCT_DEC)?;
    write_dobj_defines(&mut w)?;
    write!(w, "\n")?;
    write_ptrparam_constants(&mut w)?;
    write!(w, "\n")?;

    writeln!(w, "/* {} Macros */", cmd_type_name![]);
    for info in DynCmd::variants() {
        writeln!(w, "/* {} */", info.desc)?;
        match (info.objs.is_empty(), info.objs.is_all()) {
            (false, false) => writeln!(w, "/* Supported Objs: {} */", info.objs)?,
            (false, true)  => writeln!(w, "/* Supported Objs: all */")?,
            (true, _)      => (),
        }
        write_cmd_macros(&mut w, &info)?;
        write!(w, "\n");
    }
    writeln!(w, "{}", IFGUARD_END)?;
    Ok(())
}

#[inline]
fn write_dobj_defines<W: Write>(w: &mut W) -> Result<(), io::Error> {
    writeln!(w, "/* Object type constants for dynlist make object command */")?;
    for (constant, val) in DObjType::iter() {
        writeln!(w, "#define {} {}", constant, val)?;
    }
    Ok(())
}

#[inline]
fn write_ptrparam_constants<W: Write>(w: &mut W) -> Result<(), io::Error> {
    writeln!(w, "/* Paramters that can be set by SetParamPtr command */")?;
    for (param, val) in PtrParam::iter() {
        writeln!(w, "#define {} {}", param, val)?;
    }
    Ok(())
}

/* In C, unlike ASM, we can't rely on default arguments (beyond what's expected for struct init) */ 
fn write_cmd_macros<W: Write>(w: &mut W, cmd: &CmdInfo) -> Result<(), io::Error> {
    use self::DynArg::*;
    match cmd.kind {
        Void => writeln!(w, 
r#"#define {}{}() \
    {{ {} }}"#, 
            PREFIX, cmd.base, cmd.id),

        First => writeln!(w,
r#"#define {}{}(w1) \
    {{ {}, (void *)(w1) }}"#, 
            PREFIX, cmd.base, cmd.id),

        Second => writeln!(w,
r#"#define {}{}(w2) \
    {{ {}, 0, (void *)(w2) }}"#, 
            PREFIX, cmd.base, cmd.id),

        Both => writeln!(w,
r#"#define {}{}(w1, w2) \
    {{ {}, (void *)(w1), (void *)(w2) }}"#, 
            PREFIX, cmd.base, cmd.id), 

        SwapBoth => writeln!(w,
r#"#define {}{}(w2, w1) \
    {{ {}, (void *)(w1), (void *)(w2) }}"#, 
            PREFIX, cmd.base, cmd.id), 

        VecXYZ | VecPtr  => writeln!(w,
r#"#define {}{}(x, y, z) \
    {{ {}, 0, 0, (x), (y), (z) }}"#, 
            PREFIX, cmd.base, cmd.id), 

        VecX  => writeln!(w,
r#"#define {}{}(x) \
    {{ {}, 0, 0, (x) }}"#, 
            PREFIX, cmd.base, cmd.id),  

        VecXY  => writeln!(w,
r#"#define {}{}(x, y) \
    {{ {}, 0, 0, (x), (y) }}"#, 
            PREFIX, cmd.base, cmd.id), 

        SecVecX  => writeln!(w,
r#"#define {}{}(w2, x) \
    {{ {}, 0, (void *)(w2), (x) }}"#, 
            PREFIX, cmd.base, cmd.id), 

        ValPtr  => writeln!(w,
r#"#define {}{}(id, flags, type, offset) \
    {{ {}, (void *)(id), (void *)(type), (offset), (flags) }}"#, 
            PREFIX, cmd.base, cmd.id),
    }
}
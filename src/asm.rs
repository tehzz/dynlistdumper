use std::io::{self, Write};
use dynlist::{DynCmd, CmdInfo, DynArg, DObjType};

const PRELUDE: &'static str = r#"# DynList GNU AS Macros
# Bool Types
.set TRUE, 1
.set FALSE, 0
.set NULL, 0

# Helper macro that has all unnecessary fields set to a default value
.macro DynListCmd cmd, w1=0, w2=0, f1=0.0, f2=0.0, f3=0.0
    .4byte \cmd, \w1, \w2
    .float \f1, \f2, \f3
.endm
"#;
const BASEMAC: &'static str = "DynListCmd";

/// Write gas macros for all of the dynlist commands. This effectively produces
/// an include file that can be used to assemble a dynlist
pub fn write_macros<W: Write>(mut w: W) -> Result<(), io::Error> {
    writeln!(w, "{}", PRELUDE)?;
    write_dobj_constants(&mut w)?;
    writeln!(w, "\n# DynList Command Macros #\n")?;
    for info in DynCmd::variants() {
        writeln!(w, "# {}", info.desc)?;
        write_cmd_macro(&mut w, &info)?;
        write!(w, "\n\n");
    }

    Ok(())
}

/// Hacky function to write all 19 of the dyn object type enum
fn write_dobj_constants<W: Write>(w: &mut W) -> Result<(), io::Error> {
    writeln!(w, "# Object type constants for dynlist make object command")?;
    for (val, constant) in DObjType::iter().enumerate() {
        writeln!(w, ".set {}, {}", constant, val)?;
    }
    Ok(())
}

fn write_cmd_macro<W: Write>(w: &mut W, cmd: &CmdInfo) -> Result<(), io::Error> {
    use self::DynArg::*;
    match cmd.kind {
        Void => write!(w, 
r#".macro {}
    {} {}
.endm"#, cmd.base, BASEMAC, cmd.id),

        First => write!(w,
r#".macro {} w1
    {} {}, \w1
.endm"#, cmd.base, BASEMAC, cmd.id),

        Second => write!(w,
r#".macro {} w2
    {} {},, \w2
.endm"#, cmd.base, BASEMAC, cmd.id),

        Both => write!(w,
r#".macro {} w1, w2
    {} {}, \w1, \w2
.endm"#, cmd.base, BASEMAC, cmd.id), 

        SwapBoth => write!(w,
r#".macro {} w1, w2
    {} {}, \w2, \w1
.endm"#, cmd.base, BASEMAC, cmd.id), 

        VecXYZ | Vector  => write!(w,
r#".macro {} x, y, z
    {} {},,, \x, \y, \z
.endm"#, cmd.base, BASEMAC, cmd.id), 
    }
}
use c89;
use dynlist::{DynListIter};
use std::io::{Write, Read};
use failure::{Error, ResultExt};
use std::collections::HashMap;

/// Write out summary info for a dynlist
pub fn info<W, R>(mut wtr: W, dynlist: DynListIter<R>, offset: u64) -> Result<(), Error> 
    where W: Write, R: Read
{
    let (info, count) = dynlist.fold( 
        (HashMap::new(), 0u64), 
        |(mut map, c), cmd| {
            let cmd = cmd.expect("processing list for summarization");
            let c = c + 1;
            *map.entry(cmd.info().base).or_insert(0) += 1;
            (map, c)
        }
    );
    writeln!(wtr, "Dynlist @ {:#X}", offset)?;
    writeln!(wtr, "Total Commands: {}", count)?;
    writeln!(wtr, "Total Size: {:#x} bytes", count * 6 * 4)?;
    writeln!(wtr, "\nCommand Summary:")?;
    for (cmd, num) in info.iter() {
        writeln!(wtr, "{} : {}", cmd, num)?;
    }
    Ok(())
}

/// Write out the raw rust structs for a dynlist
pub fn raw<W, R>(mut wtr: W, dynlist: DynListIter<R>) -> Result<(), Error> 
    where W: Write, R: Read
{
    writeln!(wtr, "Starting RAW dynlist dump")?;
    for (i, cmd) in dynlist.enumerate() {
        let cmd = cmd.context("reading command from dynlist iterator")?;
        writeln!(wtr, "cmd {}: {:x?}", i, &cmd)?;
        if cmd.is_unk() { bail!("unknown dynlist command..?") }; 
    }
    writeln!(wtr, "Finished RAW dynlist dump")?;
    Ok(())
}

/// Write out a C style array for a dynlist
pub fn c<W, R>(mut wtr: W, dynlist: DynListIter<R>, address: u64) -> Result<(), Error> 
    where W: Write, R: Read
{
    let mut count = 0;
    let prefix = c89::PREFIX;
    let structname = c89::STRUCT_NAME;

    writeln!(wtr, "{} list_{:08X}[] = {{", structname, address)?;
    for cmd in dynlist {
        let cmd = cmd.context("reading command from dynlist iterator")?;
        writeln!(wtr, "\t{}{:b},", prefix, cmd)?;
        if cmd.is_unk() { bail!("unknown dynlist command") };
        count += 1;
    }
    writeln!(wtr, "}};")?;
    writeln!(wtr, "/* Total Commands: {} */", count)?;
    Ok(())
}

/// Write out a GNU AS file of macros for a dynlist
pub fn gas<W, R>(mut wtr: W, dynlist: DynListIter<R>, address: u64) -> Result<(), Error> 
    where W: Write, R: Read
{
    writeln!(wtr, "list_{:08X}:", address);
    for cmd in dynlist {
        let cmd = cmd.context("reading command from dynlist iterator")?;
        writeln!(wtr, "\t{}", &cmd)?;
        if cmd.is_unk() { bail!("unknown dynlist command..?") }; 
    }
    Ok(())
}

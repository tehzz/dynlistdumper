#[macro_use] extern crate failure;
#[macro_use] extern crate structopt;
#[macro_use] extern crate bitflags;
extern crate byteorder;
use structopt::StructOpt;
use failure::{Error, ResultExt};

mod asm;
mod c89;
mod dynlist;
use dynlist::{DynListIter};

use std::path::PathBuf;
use std::io::{self, BufReader, BufWriter, Write, Read};
use std::fs::{File, OpenOptions};
use std::num::ParseIntError;
use std::collections::HashMap;

/// A tool to help dump a binary SM64 head screen dynlist into a set of asm macros
#[derive(Debug, StructOpt)]
enum Opts {
    /// Dump a binary dynlist into a list of gas macros
    #[structopt(name="dump")]
    Dump(Dump),
    /// Create the set of gas macros needed for assembling a dumped dynlist
    #[structopt(name="asm")]
    ASM {
        #[structopt(parse(from_os_str))]
        /// output file, or stdout if not present
        output: Option<PathBuf>,
    },
    /// Create the set of cpp macros needed for initializing a dynlist cmd struct
    #[structopt(name="c")]
    C {
        #[structopt(parse(from_os_str))]
        /// output file, or stdout if not present
        output: Option<PathBuf>,
    },
}

/// Dump a binary dynlist into a list of gas or C macros
#[derive(Debug, StructOpt)]
struct Dump {
    /// input binary file to read dynlist from
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// offset to start of dynlist
    offset: Option<String>,
    /// output file, or stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
    /// print out the C macros instead of gas
    #[structopt(short = "c", long = "c-macros", raw(conflicts_with_all = "&[\"info\", \"raw\"]"))]
    c: bool,
    /// print out the raw values of cmd as a comment
    #[structopt(short = "r", long = "raw-values", raw(conflicts_with_all = "&[\"info\", \"c\"]"))]
    raw: bool,
    /// print info about a list, rather than dumping the bytes
    #[structopt(short = "i", long = "info", raw(conflicts_with_all = "&[\"raw\", \"c\"]"))]
    info: bool,
}

fn main() {
    let opts = Opts::from_args();
    //println!("{:#?}", opts);
    if let Err(e) = run(opts) {
        eprintln!("Error: {}", e);
        for c in e.iter_causes() {
            eprintln!("caused by: {}", c);
        }
        ::std::process::exit(1);
    }
}


fn run(opts: Opts) -> Result<(),Error> {
    match opts {
        Opts::Dump(dump)  => dump_dynlist(dump),
        Opts::ASM{output} => produce_asm_macros(output),
        Opts::C{output}   => produce_c_header(output),
    }
}

fn dump_dynlist(opts: Dump) -> Result<(), Error> {
    let f = File::open(opts.input).context("opening input binary file")?;
    let rdr = BufReader::new(f);
    let offset = opts.offset.as_ref()
        .map(hex_or_dec)
        .unwrap_or(Ok(0))
        .context("parsing offset into integer")?;
    let dynlist = DynListIter::from_reader(rdr, offset).context("generating dynlist iterator")?;
    let wtr = get_file_or_stdout(opts.output).context("opening output file")?;

    match (opts.info, opts.raw, opts.c) {
        (true, false, false)  => get_dynlinst_info(wtr, dynlist, offset),
        (false, true, false)  => dump_raw(wtr, dynlist),
        (false, false, true)  => dump_c(wtr, dynlist),
        (false, false, false) => dump_gas(wtr, dynlist),
        _ => bail!("Illegal combination of dump flags"),
    }
}

fn get_dynlinst_info<W, R>(mut wtr: W, dynlist: DynListIter<R>, offset: u64) -> Result<(), Error> 
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

fn dump_raw<W, R>(mut wtr: W, dynlist: DynListIter<R>) -> Result<(), Error> 
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

fn dump_c<W, R>(mut wtr: W, dynlist: DynListIter<R>) -> Result<(), Error> 
    where W: Write, R: Read
{
    let mut count = 0;
    let prefix = c89::PREFIX;
    writeln!(wtr, "[")?;
    for cmd in dynlist {
        let cmd = cmd.context("reading command from dynlist iterator")?;
        writeln!(wtr, "{}{:b},", prefix, cmd)?;
        if cmd.is_unk() { bail!("unknown dynlist command") };
        count += 1;
    }
    writeln!(wtr, "]")?;
    writeln!(wtr, "/* Total Commands: {} */", count)?;
    Ok(())
}

fn dump_gas<W, R>(mut wtr: W, dynlist: DynListIter<R>) -> Result<(), Error> 
    where W: Write, R: Read
{
    for cmd in dynlist {
        let cmd = cmd.context("reading command from dynlist iterator")?;
        writeln!(wtr, "{}", &cmd)?;
        if cmd.is_unk() { bail!("unknown dynlist command..?") }; 
    }
    Ok(())
}

fn produce_asm_macros(out: Option<PathBuf>) -> Result<(), Error> {
    let wtr = get_file_or_stdout(out).context("opening output file")?;
    asm::write_macros(wtr)?;
    Ok(())
}

fn produce_c_header(out: Option<PathBuf>) -> Result<(), Error> {
    let wtr = get_file_or_stdout(out).context("opening output file")?;
    c89::write_header(wtr)?;
    Ok(())
}

fn hex_or_dec<S>(n: S) -> Result<u64, ParseIntError>
    where S: AsRef<str>
{
    let n: &str = n.as_ref();
    let op = &n[0..2];
    
    if op == "0x" || op == "0X" { 
        u64::from_str_radix(&n[2..], 16)
    } else { 
        u64::from_str_radix(n, 10)
    }
}

fn get_file_or_stdout(out: Option<PathBuf>) -> Result<BufWriter<Box<Write>>, io::Error> {
    Ok(BufWriter::new(
        if let Some(f) = out {
            let f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(f)?;
            Box::new(f) as Box<Write>
        } else {
            Box::new(io::stdout()) as Box<Write>
        }
    ))
}

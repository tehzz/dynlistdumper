#[macro_use] extern crate failure;
#[macro_use] extern crate structopt;
extern crate byteorder;
use structopt::StructOpt;
use failure::{Error, ResultExt};

mod dynlist;
use dynlist::{DynListIter};

use std::path::PathBuf;
use std::io::{BufReader};
use std::fs::{File};
use std::num::ParseIntError;

/// Dump a binary SM64 head screen dynlist into a set of asm macros
#[derive(Debug, StructOpt)]
struct Opts {
    /// input binary file to read dynlist from
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// offset to start of dynlist
    offset: Option<String>,
    /// output asm file, or stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
    /// print out the raw values of cmd as a comment
    #[structopt(short = "r", long = "raw-values")]
    raw: bool,
}

fn main() {
    let opts = Opts::from_args();
    println!("{:#?}", opts);
    if let Err(e) = run(opts) {
        eprintln!("Error: {}", e);
        for c in e.iter_causes() {
            eprintln!("caused by: {}", c);
        }
        ::std::process::exit(1);
    }
}


fn run(opts: Opts) -> Result<(),Error> {
    let f = File::open(opts.input).context("opening input binary file")?;
    let rdr = BufReader::new(f);

    let offset = opts.offset.as_ref()
        .map(hex_or_dec)
        .unwrap_or(Ok(0))
        .context("parsing offset into integer")?;
    let dynlist = DynListIter::from_reader(rdr, offset).context("generating iterator")?;

    for (i, cmd) in dynlist.enumerate() {
        let cmd = cmd.context("reading dynlist iterator")?;
        println!("cmd {}: {:x?}", i, &cmd);
        if cmd.is_unk() { bail!("unknown dynlist command..?") }; 
    }
    
    println!("Finished reading list");
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

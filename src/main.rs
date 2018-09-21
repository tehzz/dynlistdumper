#[macro_use] extern crate failure;
#[macro_use] extern crate structopt;
extern crate byteorder;

use structopt::StructOpt;
use failure::{Error, ResultExt};
use byteorder::{ByteOrder, BE};

mod cmd;

use cmd::DynCmd;

use std::path::PathBuf;
use std::io::{Read, BufReader, Seek, SeekFrom};
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
    println!("opts\n{:#?}", opts);
    if let Err(e) = run(opts) {
        eprintln!("Error: {}", e);
        for c in e.iter_causes() {
            eprintln!("caused by: {}", c);
        }
        ::std::process::exit(1);
    }
}

#[derive(Debug)]
struct DynList {
    cmd: DynCmd,
    raw: [u32; 6],
}

impl DynList {
    fn from_bytes(buf: &[u8; 24]) -> Self {
        let mut raw = [0; 6];
        BE::read_u32_into(buf, &mut raw);
        let cmd = DynCmd::from_struct(&raw);
        DynList{raw, cmd}
    }
    fn is_end(&self) -> bool {
        match self.cmd {
            DynCmd::Stop => true,
            _ => false,
        }
    }
    fn is_unk(&self) -> bool {
        match self.cmd {
            DynCmd::Unk(_) => true,
            _ => false,
        }
    }
}

fn run(opts: Opts) -> Result<(),Error> {
    let mut buf = [0; 24];
    let file = File::open(opts.input).context("opening input binary file")?;
    let mut rdr = BufReader::new(file);
    let offset = opts.offset.as_ref().map(hex_or_dec).unwrap_or(Ok(0))
        .context("parsing offset into integer")?;
    rdr.seek(SeekFrom::Start(offset)).context("seeking to offset")?;
    // make an iterator...?
    let mut i = 0;
    loop {
        rdr.read_exact(&mut buf).context("reading 24 bytes from input")?;
        let dyncmd = DynList::from_bytes(&buf);
        println!("cmd {}: {:x?}", i, &dyncmd);
        i+=1;
        if dyncmd.is_end() { break; }
        else if dyncmd.is_unk() { bail!("unknown dynlist command..?"); }
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

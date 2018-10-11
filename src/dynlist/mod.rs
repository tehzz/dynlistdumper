use byteorder::{ByteOrder, BE};
use std::io::{self, Read, Seek, SeekFrom};
use std::fmt;

mod cmd;
mod dobj_types;
mod param_ptr;
mod objs;
pub use self::cmd::{DynCmd, CmdInfo, DynArg};
pub use self::dobj_types::DObjType;
pub use self::objs::ObjFlag;
pub use self::param_ptr::PtrParam;

#[derive(Debug)]
pub struct DynListItem  {
    cmd: DynCmd,
    raw: [u32; 6],
}

impl fmt::Display for DynListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cmd)
    }
}
/* C macro printing hack... */
impl fmt::Binary for DynListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.cmd)
    }
}

impl DynListItem  {
    fn from_bytes(buf: &[u8; 24]) -> Self {
        let mut raw = [0; 6];
        BE::read_u32_into(buf, &mut raw);
        let cmd = DynCmd::from_struct(&raw);
        DynListItem {raw, cmd}
    }
    fn is_end(&self) -> bool {
        match self.cmd {
            DynCmd::Stop => true,
            _ => false,
        }
    }
    pub fn is_unk(&self) -> bool {
        match self.cmd {
            DynCmd::Unk(_) => true,
            _ => false,
        }
    }
    pub fn info(&self) -> CmdInfo {
        self.cmd.info()
    }
}

pub struct DynListIter<R> {
    buf: [u8; 24],
    end_found: bool,
    rdr: R,
}

impl<R: Read + Seek> DynListIter<R> {
    pub fn from_reader(mut rdr: R, offset: u64) -> Result<Self, io::Error> {
        rdr.seek(SeekFrom::Start(offset))?;

        Ok(DynListIter {
            buf: [0; 24],
            end_found: false,
            rdr
        })
    }
    /*
    pub fn into_reader(self) -> R {
        self.rdr
    }
    */
}

impl<R: Read> Iterator for DynListIter<R> {
    type Item = Result<DynListItem, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_found { return None; }
        if let Err(e) = self.rdr.read_exact(&mut self.buf) {
            return Some(Err(e.into()));
        }

        let cmd = DynListItem::from_bytes(&self.buf);
        self.end_found = cmd.is_end();

        Some(Ok(cmd))
    }
}

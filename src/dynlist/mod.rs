use byteorder::{ByteOrder, BE};
use std::io::{self, Read, Seek, SeekFrom};

mod cmd;
mod dobj_types;
use self::cmd::DynCmd;

#[derive(Debug)]
pub struct DynListItem  {
    cmd: DynCmd,
    raw: [u32; 6],
}

impl DynListItem  {
    fn from_bytes(buf: &[u8; 24]) -> Self {
        let mut raw = [0; 6];
        BE::read_u32_into(buf, &mut raw);
        let cmd = DynCmd::from_struct(&raw);
        DynListItem {raw, cmd}
    }
    pub fn is_end(&self) -> bool {
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

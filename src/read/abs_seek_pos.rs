//use std::convert::From;
use std::io::{self, Error, ErrorKind, SeekFrom};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AbsSeekPos(pub u64);

impl AbsSeekPos {
    /// Result: Ok( 0 ..= end ) or Error (kind() == InvalidInput)
    pub fn offset_bounded(&self, pos: SeekFrom, end: u64) -> io::Result<AbsSeekPos> {
        let cur = self.0;
        Ok(Self(match pos {
            SeekFrom::Start(n)      => n,
            SeekFrom::End(n)        => Self::add(end, n)?,
            SeekFrom::Current(n)    => Self::add(cur, n)?,
        }))
    }

    #[allow(dead_code)] // XXX
    /// Result: Ok( 0 ..= std::u64::MAX ) or Error (kind() == InvalidInput)
    pub fn offset_unbounded(&self, pos: SeekFrom, end: u64) -> io::Result<AbsSeekPos> {
        let cur = self.0;
        Ok(Self(match pos {
            SeekFrom::Start(n)      => n,
            SeekFrom::End(n)        => Self::add(end, n)?,
            SeekFrom::Current(n)    => Self::add(cur, n)?,
        }))
    }

    fn add(cur: u64, off: i64) -> io::Result<u64> {
        Ok(if off < 0 {
            cur.checked_sub(off.unsigned_abs()).ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Attempted to seek before start of stream"))?
        } else {
            cur.checked_add(off.unsigned_abs()).ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Attempted to seek past end of stream"))?
        })
    }
}

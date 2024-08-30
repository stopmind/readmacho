use std::io::{Error, Read};
use std::mem::{size_of, transmute};

pub fn read_magic(read: &mut dyn Read) -> Result<u32, Error> {
    let mut buf = [0u8; size_of::<u32>()];
    match read.read_exact(&mut buf) {
        Ok(_) => unsafe { Ok(transmute(buf)) }
        Err(err) => Err(err)
    }
}

pub trait FromRead {
    fn read(read: &mut dyn Read) -> Result<Box<Self>, Error>;
}
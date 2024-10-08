use std::io::{Error, Read};
use std::mem::transmute;
use log::info;
use crate::serialize::FromRead;
use crate::commands::read_command;

struct Header {
    cpu_type: u32,
    cpu_subtype: u32,
    file_type: u32,
    load_commands_count: u32,
    size_load_command: u32,
    flags: u32
}

impl FromRead for Header {
    fn read(read: &mut dyn Read) -> Result<Box<Self>, Error> {
        let mut buf = [0u8; size_of::<Self>()];
        match read.read_exact(&mut buf) {
            Ok(_) => unsafe { Ok(Box::new(transmute(buf))) }
            Err(err) => Err(err)
        }
    }
}

pub fn read_macho(read: &mut dyn Read, is64: bool) {
    info!("Header.");

    let header = Header::read(read).unwrap();

    info!("CPU Type       : 0x{:08x}", header.cpu_type);
    info!("CPU Subtype    : 0x{:08x}", header.cpu_subtype);
    info!("File Type      : 0x{:08x}", header.file_type);
    info!("Commands count : {}",       header.load_commands_count);
    info!("Commands size  : {}",       header.size_load_command);
    info!("Flags          : 0x{:08x}", header.flags);

    if is64 {
        read.read_exact(&mut [0u8; size_of::<u32>()]).unwrap();
    }

    info!("Reading commands..");

    for _ in 0..header.load_commands_count {
        read_command(read);
    }
}
use std::intrinsics::transmute;
use std::io::{Error, Read};
use log::{error, info, warn};
use crate::serialize::FromRead;

struct LoadCommand {
    cmd_type: u32,
    size: u32
}

impl FromRead for LoadCommand {
    fn read(read: &mut dyn Read) -> Result<Box<Self>, Error> {
        let mut buf = [0u8; size_of::<Self>()];
        match read.read_exact(&mut buf) {
            Ok(_) => unsafe { Ok(Box::new(transmute(buf))) }
            Err(err) => Err(err)
        }
    }
}

pub fn read_command(read: &mut dyn Read) {
    let cmd = LoadCommand::read(read).unwrap();

    match cmd.cmd_type {
        0x00000001 => read_segment32(read),
        0x00000019 => read_segment64(read),
        _ => {
            read.read_exact(&mut vec![0u8; cmd.size as usize - size_of::<LoadCommand>()]).unwrap();

            if cmd.cmd_type & 0x80000000 != 0 {
                error!("Unknown command is necessary. (0x{:08x})", cmd.cmd_type)
            } else {
                warn!("Unknown command. (0x{:08x})", cmd.cmd_type)
            }
        }
    }
}

struct SegmentCommand32 {
    name: [u8; 16],
    address: u32,
    address_size : u32,
    file_offset : u32,
    size: u32,
    max_protect: u32,
    init_protect: u32,
    sections_count: u32,
    flags: u32
}

struct LoadCommand64 {
    name: [u8; 16],
    address: u64,
    address_size : u64,
    file_offset : u64,
    size: u64,
    max_protect: u32,
    init_protect: u32,
    sections_count: u32,
    flags: u32
}

impl FromRead for SegmentCommand32 {
    fn read(read: &mut dyn Read) -> Result<Box<Self>, Error> {
        let mut buf = [0u8; size_of::<Self>()];
        match read.read_exact(&mut buf) {
            Ok(_) => unsafe { Ok(Box::new(transmute(buf))) }
            Err(err) => Err(err)
        }
    }
}

impl FromRead for LoadCommand64 {
    fn read(read: &mut dyn Read) -> Result<Box<Self>, Error> {
        let mut buf = [0u8; size_of::<Self>()];
        match read.read_exact(&mut buf) {
            Ok(_) => unsafe { Ok(Box::new(transmute(buf))) }
            Err(err) => Err(err)
        }
    }
}

struct Section32 {
    name: [u8; 16],
    segment_name: [u8; 16],
    section_addr: u32,
    section_size: u32,

    file_offset: u32,
    alignment: u32,
    reloc_file_offset: u32,
    reloc_count: u32,
    flag_type: u32,

    reserved: [u32; 2]
}

struct Section64 {
    name: [u8; 16],
    segment_name: [u8; 16],
    section_addr: u64,
    section_size: u64,

    file_offset: u32,
    alignment: u32,
    reloc_file_offset: u32,
    reloc_count: u32,
    flag_type: u32,

    reserved: [u32; 3]
}

impl FromRead for Section64 {
    fn read(read: &mut dyn Read) -> Result<Box<Self>, Error> {
        let mut buf = [0u8; size_of::<Self>()];
        match read.read_exact(&mut buf) {
            Ok(_) => unsafe { Ok(Box::new(transmute(buf))) }
            Err(err) => Err(err)
        }
    }
}

impl FromRead for Section32 {
    fn read(read: &mut dyn Read) -> Result<Box<Self>, Error> {
        let mut buf = [0u8; size_of::<Self>()];
        match read.read_exact(&mut buf) {
            Ok(_) => unsafe { Ok(Box::new(transmute(buf))) }
            Err(err) => Err(err)
        }
    }
}

fn read_segment32(read: &mut dyn Read) {
    let load = SegmentCommand32::read(read).unwrap();
    info!("Segment load command. (32) ");
    info!("Name           : {}", String::from_utf8_lossy(&load.name));
    info!("Address        : 0x{:08x}", load.address);
    info!("Address size   : {}",       load.address_size);
    info!("File offset    : 0x{:08x}", load.file_offset);
    info!("Size           : {}",       load.size);
    info!("Max protect    : {:032b}",  load.max_protect);
    info!("Init protect   : {:032b}",  load.init_protect);
    info!("Flags          : {:032b}",  load.flags);
    info!("Sections count : {}",       load.sections_count);

    for _ in 0..load.sections_count {
        let section = Section32::read(read).unwrap();

        info!("-- Section.");
        info!("Name                   : {}", String::from_utf8_lossy(&section.name));
        info!("Address                : 0x{:08x}", section.section_addr);
        info!("Size                   : {}", section.section_size);
        info!("File offset            : 0x{:08x}", section.file_offset);
        info!("Alignment              : {}", section.alignment);
        info!("Relocation file offset : 0x{:08x}", section.reloc_file_offset);
        info!("Relocations count      : {}", section.reloc_count);
        info!("Flag/Type              : {:032b}", section.flag_type);
    }
}

fn read_segment64(read: &mut dyn Read) {
    let load = LoadCommand64::read(read).unwrap();
    info!("Segment load command. (64) ");
    info!("Name           : {}", String::from_utf8_lossy(&load.name));
    info!("Address        : 0x{:016x}", load.address);
    info!("Address size   : {}",        load.address_size);
    info!("File offset    : 0x{:016x}", load.file_offset);
    info!("Size           : {}",        load.size);
    info!("Max protect    : {:032b}",   load.max_protect);
    info!("Init protect   : {:032b}",   load.init_protect);
    info!("Flags          : {:032b}",   load.flags);
    info!("Sections count : {}",        load.sections_count);

    for _ in 0..load.sections_count {
        let section = Section64::read(read).unwrap();

        info!("--      Section.");
        info!("Name                   : {}", String::from_utf8_lossy(&section.name));
        info!("Address                : 0x{:016x}", section.section_addr);
        info!("Size                   : {}", section.section_size);
        info!("File offset            : 0x{:08x}", section.file_offset);
        info!("Alignment              : {}", section.alignment);
        info!("Relocation file offset : 0x{:08x}", section.reloc_file_offset);
        info!("Relocations count      : {}", section.reloc_count);
        info!("Flag/Type              : {:032b}", section.flag_type);
    }
}
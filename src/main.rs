mod serialize;
mod macho;

use std::env::args;
use env_logger::{init, Builder};
use log::*;
use std::io::Write;
use std::io::Read;
use core::*;
use crate::macho::read_macho;
use crate::serialize::read_magic;

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}",
                {
                    match record.level() {
                        Level::Debug => "DBUG",
                        Level::Error => "FAIL",
                        Level::Warn => "WARN",
                        Level::Info => "INFO",
                        _ => " -- "
                    }
                },
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        error!("Specify the file.");
        return;
    }

    let mut file = std::fs::File::open(&args[1]).unwrap();

    match read_magic(&mut file).unwrap() {
        0xfeedface => {
            info!("Detected Mach-O (32) magic.");
            read_macho(&mut file, false);
        }
        0xfeedfacf => {
            info!("Detected Mach-O (64) magic.");
            read_macho(&mut file, true);
        }
        _ => error!("Unknown magic!")
    }
}
use binrw::binread;
use binrw::helpers::until_exclusive;
use binrw::prelude::*;
use std::env;
use std::fs::File;

#[binread]
#[derive(Debug)]
struct EqTable {
    #[br(parse_with = until_exclusive(|v: &EqEntry| v.cpu == 0))]
    entries: Vec<EqEntry>,
}

#[binread]
#[derive(Debug)]
struct EqEntry {
    cpu: u32,
    _mask: u32,
    _comp: u32,
    equiv: u16,
    _rsvd: u16,
}

impl EqEntry {
    pub fn family(&self) -> u8 {
        (((self.cpu >> 8) & 0xf) + ((self.cpu >> 20) & 0xff)) as u8
    }
    pub fn model(&self) -> u8 {
        (((self.cpu >> 4) & 0xf) | ((self.cpu >> 12) & 0xf0)) as u8
    }
    pub fn stepping(&self) -> u8 {
        (self.cpu & 0xf) as u8
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let table: EqTable = file.read_le().unwrap();

    for e in &table.entries {
        println!("Signature={:#010x} Family={:#04x} Model={:#04x} Stepping={:#04x} -> {:4X}",
            e.cpu, e.family(), e.model(), e.stepping(), e.equiv);
    }
}


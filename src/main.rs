extern crate ff6;
extern crate toml;

use ff6::events;
use ff6::field;
use std::fs::File;
use std::fs::{create_dir_all, write};
use std::io::Error;
use std::io::Read;

struct Rom {
    data: Vec<u8>,
}

impl Rom {
    fn new() -> Result<Rom, Error> {
        let mut f = File::open("ff3.sfc")?;
        let mut rom: Rom = Rom { data: Vec::new() };
        f.read_to_end(&mut rom.data)?;
        Ok(rom)
    }

    fn parse_locations(&self) {
        let locations = field::parse(&self.data).unwrap();
        create_dir_all("out/field/").unwrap();
        for l in 0..locations.len() {
            let t = toml::to_string(&locations[l]).unwrap();
            write(format!("out/field/{:03x}.toml", l), t).unwrap();
        }
    }
}

fn main() -> std::io::Result<()> {
    let rom = Rom::new()?;

    rom.parse_locations();
    Ok(())
}

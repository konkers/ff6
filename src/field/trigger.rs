use simple_error::SimpleError;
use std::error::Error;

use ptr_table;
use rom_map;
use utils::get_u24;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Trigger {
    x: u8,
    y: u8,
    event_addr: u32,
}

pub fn ptr_table(rom_data: &[u8]) -> Result<ptr_table::Table, Box<Error>> {
    // TODO bounds check rom data.
    let addr = rom_map::snes_to_file(rom_map::EVENT_TRIGGER_POINTERS) as usize;
    let table = ptr_table::Table::new(
        &rom_data[addr..],
        0x1a0,
        rom_map::snes_to_file(rom_map::EVENT_TRIGGER_POINTERS),
    );
    Ok(table)
}

pub fn parse_trigger(data: &[u8]) -> Result<Trigger, Box<Error>> {
    if data.len() < 0x5 {
        return Err(SimpleError::new("data does not contain at least 0x5 bytes").into());
    }

    Ok(Trigger {
        x: data[0],
        y: data[1],
        event_addr: get_u24(&data[2..]),
    })
}

pub fn parse_triggers(data: &[u8]) -> Result<Vec<Trigger>, Box<Error>> {
    let num = data.len() / 5;
    let mut npcs = Vec::new();
    for i in 0..num {
        npcs.push(parse_trigger(&data[(i * 5)..])?);
    }
    Ok(npcs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let data = [0x40, 0x0e, 0xf2, 0x38, 0x02, 0x08, 0x2e, 0xff, 0x38, 0x02];
        let triggers = vec![
            Trigger {
                x: 64,
                y: 14,
                event_addr: 145650,
            },
            Trigger {
                x: 8,
                y: 46,
                event_addr: 145663,
            },
        ];

        assert_eq!(triggers, parse_triggers(&data).unwrap());
    }
}

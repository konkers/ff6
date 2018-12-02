use simple_error::SimpleError;
use std::error::Error;

use ptr_table;
use rom_map;
use utils::{get_u16, get_u24, test_bit};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Npc {
    event_addr: u32,
    palette: u8,
    solid_action_path: bool,

    // bit and byte offset info the NPC Event bits at $1EE0.  The bit
    // specified controls NPC visibility.
    enable_bit: u8,
    enable_addr: u8,

    x: u8,
    show_rider_in_vehicle: bool,
    y: u8,
    speed: u8,
    sprite: u8,
    movement_type: u8, // ZD CE: action
    map_layer: u8,     // ZD CE: walkUnder, walkOver
    vehicle: u8,
    start_direction: u8,       // ZD CE: f
    turn_when_triggered: bool, // ZD CE: dontFaceOnTrigger
    unknown_8_bits: u8,
}

pub fn ptr_table(rom_data: &[u8]) -> Result<ptr_table::Table, Box<Error>> {
    // TODO bounds check rom data.
    let addr = rom_map::snes_to_file(rom_map::NPC_POINTERS);
    let table = ptr_table::Table::new(
        &rom_data[addr..],
        0x1a0,
        rom_map::snes_to_file(rom_map::NPC_POINTERS),
    );
    Ok(table)
}

pub fn parse_npc(data: &[u8]) -> Result<Npc, Box<Error>> {
    if data.len() < 0x9 {
        return Err(SimpleError::new("data does not contain at least 0x9 bytes").into());
    }

    Ok(Npc {
        event_addr: get_u24(data) & 0x3ffff,
        palette: (data[2] >> 2) & 0x7,
        solid_action_path: test_bit(data[2], 5),
        enable_bit: ((get_u16(&data[2..]) >> 6) & 0x7) as u8,
        enable_addr: data[3] >> 1,
        x: data[4] & 0x7f,
        show_rider_in_vehicle: test_bit(data[4], 7),
        y: data[5] & 0x3f,
        speed: data[5] >> 6,
        sprite: data[6],
        movement_type: data[7] & 0xf,
        map_layer: (data[7] >> 4) & 0x3,
        vehicle: (data[7] >> 6) & 0x3,
        start_direction: data[8] & 0x3,
        turn_when_triggered: test_bit(data[8], 2),
        unknown_8_bits: data[8] >> 3,
    })
}

pub fn parse_npcs(data: &[u8]) -> Result<Vec<Npc>, Box<Error>> {
    let num = data.len() / 9;
    let mut npcs = Vec::new();
    for i in 0..num {
        npcs.push(parse_npc(&data[(i * 9)..])?);
    }
    Ok(npcs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let npcs = vec![
            Npc {
                event_addr: 184951,
                palette: 1,
                solid_action_path: false,
                enable_bit: 6,
                enable_addr: 96,
                x: 64,
                show_rider_in_vehicle: false,
                y: 7,
                speed: 1,
                sprite: 54,
                movement_type: 0,
                map_layer: 0,
                vehicle: 0,
                start_direction: 2,
                turn_when_triggered: false,
                unknown_8_bits: 0,
            },
            Npc {
                event_addr: 184999,
                palette: 1,
                solid_action_path: false,
                enable_bit: 6,
                enable_addr: 96,
                x: 8,
                show_rider_in_vehicle: false,
                y: 38,
                speed: 1,
                sprite: 54,
                movement_type: 0,
                map_layer: 0,
                vehicle: 0,
                start_direction: 2,
                turn_when_triggered: false,
                unknown_8_bits: 0,
            },
        ];
        let data = [
            0x77, 0xd2, 0x06, 0xc0, 0x40, 0x47, 0x36, 0x00, 0x02, 0xa7, 0xd2, 0x06, 0xc0, 0x08,
            0x66, 0x36, 0x00, 0x02,
        ];
        assert_eq!(npcs, parse_npcs(&data).unwrap());
    }
}

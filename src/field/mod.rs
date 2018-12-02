use std::error::Error;

use rom_map;
use utils::get_u24;

pub mod npc;
pub mod properties;
pub mod trigger;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Location {
    properties: properties::Properties,
    entrance_event_addr: u32,
    triggers: Vec<trigger::Trigger>,
    npcs: Vec<npc::Npc>,
}

pub fn parse(rom_data: &[u8]) -> Result<Vec<Location>, Box<Error>> {
    let mut locs = Vec::new();
    let npc_table = npc::ptr_table(&rom_data)?;
    let trigger_table = trigger::ptr_table(&rom_data)?;

    for l in 0..0x19f {
        let properties = properties::parse(properties::data(l, &rom_data)?)?;

        let entrance_table = rom_map::snes_to_file(rom_map::LOCATION_ENTRANCE_EVENTS) + l * 3;
        let entrance_event = get_u24(&rom_data[entrance_table..]);

        let npc_entry = &npc_table.entries[l];
        let npcs = npc::parse_npcs(npc_entry.slice(&rom_data)?)?;

        let trigger_entry = &trigger_table.entries[l];
        let triggers = trigger::parse_triggers(trigger_entry.slice(&rom_data)?)?;

        locs.push(Location {
            properties: properties,
            entrance_event_addr: entrance_event,
            triggers: triggers,
            npcs: npcs,
        });
    }

    Ok(locs)
}

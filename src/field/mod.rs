use std::error::Error;

pub mod npc;
pub mod properties;
pub mod trigger;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Location {
    properties: properties::Properties,
    triggers: Vec<trigger::Trigger>,
    npcs: Vec<npc::Npc>,
}

pub fn parse(rom_data: &[u8]) -> Result<Vec<Location>, Box<Error>> {
    let mut locs = Vec::new();
    let npc_table = npc::ptr_table(&rom_data)?;
    let trigger_table = trigger::ptr_table(&rom_data)?;

    for l in 0..0x19f {
        let properties = properties::parse(properties::data(l, &rom_data)?)?;

        let npc_entry = &npc_table.entries[l];
        let npcs = npc::parse_npcs(npc_entry.slice(&rom_data)?)?;

        let trigger_entry = &trigger_table.entries[l];
        let triggers = trigger::parse_triggers(trigger_entry.slice(&rom_data)?)?;

        locs.push(Location {
            properties: properties,
            triggers: triggers,
            npcs: npcs,
        });
    }

    Ok(locs)
}

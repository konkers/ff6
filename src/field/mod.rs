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

        // TODO: this leaks the size of npc and trigger records.  PtrTable
        // should be refactored to take a record length and keep record count
        // in its entry.
        let npc_entry = &npc_table.entries[l];
        let npcs = npc::parse_npcs(&rom_data[(npc_entry.addr as usize)..], npc_entry.len / 9)?;

        let trigger_entry = &trigger_table.entries[l];
        let triggers = trigger::parse_triggers(
            &rom_data[(trigger_entry.addr as usize)..],
            trigger_entry.len / 5,
        )?;

        locs.push(Location {
            properties: properties,
            triggers: triggers,
            npcs: npcs,
        });
    }

    Ok(locs)
}

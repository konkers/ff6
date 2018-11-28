use std::error::Error;

pub mod npc;
pub mod properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Location {
    properties: properties::Properties,
    npcs: Vec<npc::Npc>,
}

pub fn parse(rom_data: &[u8]) -> Result<Vec<Location>, Box<Error>> {
    let mut locs = Vec::new();
    let npc_table = npc::ptr_table(&rom_data)?;

    for l in 0..0x19f {
        let properties = properties::parse(properties::data(l, &rom_data)?)?;
        let entry = &npc_table.entries[l];
        let npcs = npc::parse_npcs(&rom_data[(entry.addr as usize)..], entry.len/9)?;
        locs.push(Location {
            properties: properties,
            npcs: npcs,
        });
    }

    Ok(locs)
}

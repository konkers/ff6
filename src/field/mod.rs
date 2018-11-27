use std::error::Error;

pub mod properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Location {
    properties: properties::Properties,
}

pub fn parse(rom_data: &[u8]) -> Result<Vec<Location>, Box<Error>> {
    let mut locs = Vec::new();

    for l in 0..0x19f {
        locs.push(Location {
            properties: properties::parse(properties::data(l, &rom_data)?)?,
        });
    }

    Ok(locs)
}

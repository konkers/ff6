pub const NPC_POINTERS: u32 = 0xc41a10;
pub const NPC_DATA: u32 = 0xc41d52;
pub const LOCATION_PROPERTIES: u32 = 0xed8f00;

pub const SNES_ROM_ADDR: u32 = 0xc00000;

pub fn snes_to_file(snes_addr: u32) -> u32 {
    return snes_addr - SNES_ROM_ADDR;
}

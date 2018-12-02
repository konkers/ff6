pub const EVENT_TRIGGER_POINTERS: usize = 0xC40000;
pub const NPC_POINTERS: usize = 0xc41a10;
pub const NPC_DATA: usize = 0xc41d52;
pub const LOCATION_PROPERTIES: usize = 0xed8f00;

pub const SNES_ROM_ADDR: usize = 0xc00000;

pub fn snes_to_file(snes_addr: usize) -> usize {
    return snes_addr - SNES_ROM_ADDR;
}

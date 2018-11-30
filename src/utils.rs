pub fn get_u16(data: &[u8]) -> u32 {
    (data[0] as u32) | ((data[1] as u32) << 1)
}

pub fn get_u24(data: &[u8]) -> u32 {
    (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16)
}

pub fn test_bit(data: u8, bit: u8) -> bool {
    let mask = 1 << bit;
    return (data & mask) == mask;
}

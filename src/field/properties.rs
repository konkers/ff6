use simple_error::SimpleError;
use std::error::Error;

use rom_map;

#[derive(Debug, PartialEq, Clone)]
pub enum BgDimension {
    Bg256,
    Bg512,
    Bg1024,
    Bg2048,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Properties {
    name_index: u8,
    enable_x_zone: bool,
    enable_warp: bool,
    wavy_bg3: bool,
    wavy_bg2: bool,
    wavy_bg1: bool,
    unknown_flag_1_6: bool,
    enable_spotlights: bool,
    load_timer_graphics: bool,
    battle_background: u8,
    bg3_in_foreground: bool,
    unknown3: u8,
    tile_properties_index: u8,
    battle_properties: u8,
    enable_random_battles: bool,
    window_mask_settings: u8, // 2 bits?
    unknown_6_bits: u8,
    colosseum_house: bool,
    bg1_bg2_graphics: [u8; 4],
    bg3_graphics_index: u8,
    bg1_tileset_index: u8,
    bg2_tileset_index: u8,
    bg_tilemap_index: [u16; 3],
    sprite_overlay_index: u8,
    bg2_shift_left: u8,
    bg2_shift_up: u8,
    bg3_shift_left: u8,
    bg3_shift_up: u8,
    bg2_bg3_scroll_mode: u8,
    bg1_h: BgDimension,
    bg1_w: BgDimension,
    bg2_h: BgDimension,
    bg2_w: BgDimension,
    bg3_h: BgDimension,
    bg3_w: BgDimension,
    unused_18_bits: u8,
    palette_index: u8,
    palette_animation_index: u8,
    bg1_bg2_animation_index: u8,
    bg3_animation_index: u8,
    music_track: u8,
    unknown_1d: u8,
    map_width: u8,
    map_height: u8,
    bg2_bg3_color_math_mode: u8,
}

fn test_bit(data: u8, bit: u8) -> bool {
    let mask = 1 << bit;
    return (data & mask) == mask;
}

fn bg_dim(n: u8) -> BgDimension {
    match n & 0x3 {
        0x0 => BgDimension::Bg256,
        0x1 => BgDimension::Bg512,
        0x2 => BgDimension::Bg1024,
        _ => BgDimension::Bg2048,
    }
}

pub fn data(index: usize, rom_data: &[u8]) -> Result<&[u8], Box<Error>> {
    if index >= 0x19f {
        return Err(SimpleError::new("Index larger than 0x19e").into());
    }

    let addr = index * 0x21 + rom_map::snes_to_file(rom_map::LOCATION_PROPERTIES) as usize;
    Ok(&rom_data[addr..])
}

pub fn parse(data: &[u8]) -> Result<Properties, Box<Error>> {
    if data.len() < 0x21 {
        return Err(SimpleError::new("data does not contain at least 0x21 bytes").into());
    }

    Ok(Properties {
        name_index: data[0x0],
        enable_x_zone: test_bit(data[0x01], 0),
        enable_warp: test_bit(data[0x01], 1),
        wavy_bg3: test_bit(data[0x01], 2),
        wavy_bg2: test_bit(data[0x01], 3),
        wavy_bg1: test_bit(data[0x01], 4),
        enable_spotlights: test_bit(data[0x01], 5),
        unknown_flag_1_6: test_bit(data[0x01], 6),
        load_timer_graphics: test_bit(data[0x01], 7),
        battle_background: data[0x02] & 0x7f,
        bg3_in_foreground: test_bit(data[0x02], 7),
        unknown3: data[0x03],
        tile_properties_index: data[0x04],
        battle_properties: data[0x05] & 0x7f,
        enable_random_battles: test_bit(data[0x05], 7),
        window_mask_settings: data[0x6] & 0x3,
        unknown_6_bits: (data[0x6] >> 2) & 0x1f,
        colosseum_house: test_bit(data[0x06], 7),
        bg1_bg2_graphics: [
            data[0x07] & 0x7f,
            ((data[0x07] >> 7) | (data[0x08] << 1)) & 0x7f,
            ((data[0x08] >> 6) | (data[0x09] << 2)) & 0x7f,
            ((data[0x09] >> 5) | (data[0x0a] << 3)) & 0x7f,
        ],
        bg3_graphics_index: ((data[0x0a] >> 4) | (data[0x0b] << 4)) & 0x3f,
        bg1_tileset_index: ((data[0x0b] >> 2) | (data[0x0c] << 6)) & 0x7f,
        bg2_tileset_index: data[0x0c] >> 1,
        bg_tilemap_index: [
            (data[0x0d] as u16 | (data[0x0e] as u16) << 8) & 0xfff,
            ((data[0x0e] as u16) >> 2 | (data[0x0f] as u16) << 6) & 0xfff,
            ((data[0x0f] as u16) >> 4 | (data[0x10] as u16) << 4) & 0xfff,
        ],
        sprite_overlay_index: data[0x11],
        bg2_shift_left: data[0x12],
        bg2_shift_up: data[0x13],
        bg3_shift_left: data[0x14],
        bg3_shift_up: data[0x15],
        bg2_bg3_scroll_mode: data[0x16],
        bg1_h: bg_dim(data[0x17]),
        bg1_w: bg_dim(data[0x17] >> 2),
        bg2_h: bg_dim(data[0x17] >> 4),
        bg2_w: bg_dim(data[0x17] >> 6),
        bg3_h: bg_dim(data[0x18] >> 4),
        bg3_w: bg_dim(data[0x18] >> 6),
        unused_18_bits: data[0x18] & 0xf,
        palette_index: data[0x19],
        palette_animation_index: data[0x1a],
        bg1_bg2_animation_index: data[0x1b] & 0x1f,
        bg3_animation_index: data[0x1b] >> 5,
        music_track: data[0x1c],
        unknown_1d: data[0x1d],
        map_width: data[0x1e],
        map_height: data[0x1f],
        bg2_bg3_color_math_mode: data[0x20],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let data = [
            0x00, 0x00, 0x30, 0x00, 0x13, 0x80, 0x00, 0xaa, 0xc8, 0x06, 0x00, 0x64, 0x34, 0x03,
            0x01, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x0f, 0x00, 0x00,
            0x00, 0x00, 0x1f, 0x0e, 0x00,
        ];
        assert_eq!(
            Properties {
                name_index: 0,
                enable_x_zone: false,
                enable_warp: false,
                wavy_bg3: false,
                wavy_bg2: false,
                wavy_bg1: false,
                unknown_flag_1_6: false,
                enable_spotlights: false,
                load_timer_graphics: false,
                battle_background: 48,
                bg3_in_foreground: false,
                unknown3: 0,
                tile_properties_index: 19,
                battle_properties: 0,
                enable_random_battles: true,
                window_mask_settings: 0,
                unknown_6_bits: 0,
                colosseum_house: false,
                bg1_bg2_graphics: [42, 17, 27, 0],
                bg3_graphics_index: 0,
                bg1_tileset_index: 25,
                bg2_tileset_index: 26,
                bg_tilemap_index: [259, 0, 0],
                sprite_overlay_index: 9,
                bg2_shift_left: 0,
                bg2_shift_up: 0,
                bg3_shift_left: 0,
                bg3_shift_up: 0,
                bg2_bg3_scroll_mode: 0,
                bg1_h: BgDimension::Bg256,
                bg1_w: BgDimension::Bg256,
                bg2_h: BgDimension::Bg256,
                bg2_w: BgDimension::Bg256,
                bg3_h: BgDimension::Bg256,
                bg3_w: BgDimension::Bg256,
                unused_18_bits: 15,
                palette_index: 15,
                palette_animation_index: 0,
                bg1_bg2_animation_index: 0,
                bg3_animation_index: 0,
                music_track: 0,
                unknown_1d: 0,
                map_width: 31,
                map_height: 14,
                bg2_bg3_color_math_mode: 0
            },
            parse(&data).unwrap()
        );
    }
}

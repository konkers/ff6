use nom::{le_u16, le_u24, le_u8, ErrorKind};

#[derive(Debug, PartialEq)]
enum Tag {
    MoveDiagRightUp1x1 = 0xa0,
    MoveDiagRightDown1x1 = 0xa1,
    MoveDiagLeftDown1x1 = 0xa2,
    MoveDiagLeftUp1x1 = 0xa3,
    MoveDiagRightUp1x2 = 0xa4,
    MoveDiagRightUp2x1 = 0xa5,
    MoveDiagRightDown2x1 = 0xa6,
    MoveDiagRightDown1x2 = 0xa7,
    MoveDiagLeftDown1x2 = 0xa8,
    MoveDiagLeftDown2x1 = 0xa9,
    MoveDiagLeftUp2x1 = 0xaa,
    MoveDiagLeftUp1x2 = 0xab,

    ConditionJumpAnd1 = 0xb0,
    ConditionJumpAnd2 = 0xb1,
    ConditionJumpAnd3 = 0xb2,
    ConditionJumpAnd4 = 0xb3,
    ConditionJumpAnd5 = 0xb4,
    ConditionJumpAnd6 = 0xb5,
    ConditionJumpAnd7 = 0xb6,
    ConditionJumpAnd8 = 0xb7,
    ConditionJumpOr1 = 0xb8,
    ConditionJumpOr2 = 0xb9,
    ConditionJumpOr3 = 0xba,
    ConditionJumpOr4 = 0xbb,
    ConditionJumpOr5 = 0xbc,
    ConditionJumpOr6 = 0xbd,
    ConditionJumpOr7 = 0xbe,
    ConditionJumpOr8 = 0xbf,

    SetEntitySpeedSlowest = 0xc0,
    SetEntitySpeedSlow = 0xc1,
    SetEntitySpeedNormal = 0xc2,
    SetEntitySpeedFast = 0xc3,
    SetEntitySpeedFastest = 0xc4,

    UnknownCmdC7 = 0xc7,
    SetEventBit = 0xc8,
    ClearEventBit = 0xc9,

    TurnCharacterUp = 0xcc,
    TurnCharacterRight = 0xcd,
    TurnCharacterDown = 0xce,
    TurnCharacterLeft = 0xcf,

    ShowCharacter = 0xd0,
    HideCharacter = 0xd1,
    LoadMap = 0xd2,
    LoadMap2 = 0xd3,

    // TODO:
    // D4 aaaaaa                           If ($08 & 0x80 == 0), goto $aaaaaa
    // D5 xx aaaaaa                        If ($F6 != xx), goto $aaaaaa
    UnfadeScreen = 0xd8,
    FadeScreen = 0xd9,

    HideMiniMap = 0xdd,
    ShowMiniMap = 0xdf,

    Pause = 0xe0,

    // ChangeToShipSprite = 0xfc,
    // ShowFigaroSubmerging = 0xfd,
    // ShowFigaroEmerging = 0xfe,
    End = 0xff,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    byte: u16,
    bit: u8,
    is_set: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CondOp {
    Or,
    And,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Diagonal {
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Speed {
    Slowest,
    Slow,
    Normal,
    Fast,
    Fastest,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    ClrSetEventBit {
        set: bool,
        byte: u16,
        bit: u8,
    },
    ConditionalJump {
        op: CondOp,
        conditions: Vec<Condition>,
        addr: u32,
    },
    EntitySpeed {
        speed: Speed,
    },
    FadeScreen,
    GraphicalAction {
        action: u8,
        flipped: bool,
    },
    HideCharacter,
    HideMiniMap,
    // TODO: there are flags in LoadMap we're not parsing.
    LoadMap {
        map: u16,
        x: u8,
        y: u8,
        mode: u8,
        variant: u8,
    },
    Move {
        dir: u8,
        steps: u8,
    }, // TODO: make dir an enum.
    MoveDiag {
        dir: Diagonal,
        steps: [u8; 2],
    },
    Pause {
        frames: u8,
    },
    ShowCharacter,
    ShowMiniMap,
    TurnCharacter {
        dir: Direction,
    },
    UnfadeScreen,
    UnknownCmdC7 {
        args: [u8; 2],
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Script {
    events: Vec<Event>,
}

named!(parse_clear_set_event_bit<&[u8], Event>,
    do_parse!(
        op: parse_range!(Tag::SetEventBit, Tag::ClearEventBit) >>
        v: le_u16 >>
        (Event::ClrSetEventBit{set: op[0] == Tag::SetEventBit as u8, byte: v >> 3, bit: (v & 0x7) as u8})
    ));

named!(parse_condition<&[u8], Condition>,
    do_parse!(
        v: le_u16 >>
        (Condition{byte: (v >> 3) & 0xfff, bit: (v & 0x7) as u8, is_set: (v >> 15) == 0x1})
    ));

macro_rules! parse_conditional_jmp {
    ($i:expr, $tag:expr, $op:expr, $n:expr) => {
        do_parse!(
            $i,
            tag!(&[$tag as u8])
                >> c: count!(parse_condition, $n)
                >> addr: le_u24
                >> (Event::ConditionalJump {
                    op: $op,
                    conditions: c,
                    addr: addr
                })
        )
    };
}

named!(parse_graphical_action<&[u8], Event>,
    do_parse!(
        v: parse_range!(0x0, 0x7F) >>
        (Event::GraphicalAction{action: v[0] & 0x3f, flipped: (v[0] >> 6) == 1})
    ));

named!(parse_load_map<&[u8], Event>,
    do_parse!(
        tag: parse_range!(Tag::LoadMap, Tag::LoadMap2) >>
        map: le_u16 >>
        x: le_u8 >>
        y: le_u8 >>
        mode: le_u8 >>
        (Event::LoadMap{map: map, x: x, y: y, mode: mode, variant: tag[0]})
            ));

named!(parse_movement<&[u8], Event>,
    do_parse!(
        v: parse_range!(0x80, 0x9F) >>
            (Event::Move{steps: ((v[0] >> 2)& 0x7),
             dir: v[0] & 0x3})
    ));

named!(parse_pause<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::Pause as u8]) >>
        frames: le_u8 >>
        (Event::Pause{frames: frames})
    ));

named!(parse_unknown_cmd_c7<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::UnknownCmdC7 as u8]) >>
        args: take!(2) >>
        (Event::UnknownCmdC7{args: [args[0], args[1]]})
    ));

named!(pub parse_event<&[u8], Event>, alt!(
    parse_simple_event!(Tag::SetEntitySpeedSlowest,
        Event::EntitySpeed{speed: Speed::Slowest}) |
    parse_simple_event!(Tag::SetEntitySpeedSlow,
        Event::EntitySpeed{speed: Speed::Slow}) |
    parse_simple_event!(Tag::SetEntitySpeedNormal,
        Event::EntitySpeed{speed: Speed::Normal}) |
    parse_simple_event!(Tag::SetEntitySpeedFast,
        Event::EntitySpeed{speed: Speed::Fast}) |
    parse_simple_event!(Tag::SetEntitySpeedFastest,
        Event::EntitySpeed{speed: Speed::Fastest}) |

    parse_simple_event!(Tag::FadeScreen, Event::FadeScreen) |

    parse_simple_event!(Tag::HideCharacter, Event::HideCharacter) |
    parse_simple_event!(Tag::HideMiniMap, Event::HideMiniMap) |

    parse_simple_event!(Tag::MoveDiagRightUp1x1,
        Event::MoveDiag{dir: Diagonal::RightUp, steps: [1, 1]}) |
    parse_simple_event!(Tag::MoveDiagRightDown1x1,
        Event::MoveDiag{dir: Diagonal::RightDown, steps: [1, 1]}) |
    parse_simple_event!(Tag::MoveDiagLeftDown1x1,
        Event::MoveDiag{dir: Diagonal::LeftDown, steps: [1, 1]}) |
    parse_simple_event!(Tag::MoveDiagLeftUp1x1,
        Event::MoveDiag{dir: Diagonal::LeftUp, steps: [1, 1]}) |
    parse_simple_event!(Tag::MoveDiagRightUp1x2,
        Event::MoveDiag{dir: Diagonal::RightUp, steps: [1, 2]}) |
    parse_simple_event!(Tag::MoveDiagRightUp2x1,
        Event::MoveDiag{dir: Diagonal::RightUp, steps: [2, 1]}) |
    parse_simple_event!(Tag::MoveDiagRightDown2x1,
        Event::MoveDiag{dir: Diagonal::RightDown, steps: [2, 1]}) |
    parse_simple_event!(Tag::MoveDiagRightDown1x2,
        Event::MoveDiag{dir: Diagonal::RightDown, steps: [1, 2]}) |
    parse_simple_event!(Tag::MoveDiagLeftDown1x2,
        Event::MoveDiag{dir: Diagonal::LeftDown, steps: [1, 2]}) |
    parse_simple_event!(Tag::MoveDiagLeftDown2x1,
        Event::MoveDiag{dir: Diagonal::LeftDown, steps: [2, 1]}) |
    parse_simple_event!(Tag::MoveDiagLeftUp2x1,
        Event::MoveDiag{dir: Diagonal::LeftUp, steps: [2, 1]}) |
    parse_simple_event!(Tag::MoveDiagLeftUp1x2,
        Event::MoveDiag{dir: Diagonal::LeftUp, steps: [1, 2]}) |

    parse_simple_event!(Tag::ShowCharacter, Event::ShowCharacter) |
    parse_simple_event!(Tag::ShowMiniMap, Event::ShowMiniMap) |

    parse_simple_event!(Tag::TurnCharacterUp,
        Event::TurnCharacter{dir: Direction::Up}) |
    parse_simple_event!(Tag::TurnCharacterRight,
        Event::TurnCharacter{dir: Direction::Right}) |
    parse_simple_event!(Tag::TurnCharacterDown,
        Event::TurnCharacter{dir: Direction::Down}) |
    parse_simple_event!(Tag::TurnCharacterLeft,
        Event::TurnCharacter{dir: Direction::Left}) |

    parse_simple_event!(Tag::UnfadeScreen, Event::UnfadeScreen) |

    parse_conditional_jmp!(Tag::ConditionJumpAnd1, CondOp::And, 1) |
    parse_conditional_jmp!(Tag::ConditionJumpAnd2, CondOp::And, 2) |
    parse_conditional_jmp!(Tag::ConditionJumpAnd3, CondOp::And, 3) |
    parse_conditional_jmp!(Tag::ConditionJumpAnd4, CondOp::And, 4) |
    parse_conditional_jmp!(Tag::ConditionJumpAnd5, CondOp::And, 5) |
    parse_conditional_jmp!(Tag::ConditionJumpAnd6, CondOp::And, 6) |
    parse_conditional_jmp!(Tag::ConditionJumpAnd7, CondOp::And, 7) |
    parse_conditional_jmp!(Tag::ConditionJumpAnd8, CondOp::And, 8) |
    parse_conditional_jmp!(Tag::ConditionJumpOr1, CondOp::Or, 1) |
    parse_conditional_jmp!(Tag::ConditionJumpOr2, CondOp::Or, 2) |
    parse_conditional_jmp!(Tag::ConditionJumpOr3, CondOp::Or, 3) |
    parse_conditional_jmp!(Tag::ConditionJumpOr4, CondOp::Or, 4) |
    parse_conditional_jmp!(Tag::ConditionJumpOr5, CondOp::Or, 5) |
    parse_conditional_jmp!(Tag::ConditionJumpOr6, CondOp::Or, 6) |
    parse_conditional_jmp!(Tag::ConditionJumpOr7, CondOp::Or, 7) |
    parse_conditional_jmp!(Tag::ConditionJumpOr8, CondOp::Or, 8) |

    parse_clear_set_event_bit |
    parse_graphical_action |
    parse_load_map |
    parse_movement |
    parse_pause |
    parse_unknown_cmd_c7 |

    return_error!(ErrorKind::Custom(42),
        parse_simple_event!(Tag::FadeScreen, Event::FadeScreen))
));

named!(pub parse_script<&[u8], Script>, do_parse!(
    events: many_till!(parse_event,
    tag!(&[Tag::End as u8])) >>
    (Script{events: events.0})));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_events_test() {
        assert_eq!(
            Event::ClrSetEventBit {
                set: true,
                byte: 0x39,
                bit: 0x4
            },
            parse_event(&[Tag::SetEventBit as u8, 0xcc, 0x01])
                .unwrap()
                .1
        );

        assert_eq!(
            Event::EntitySpeed {
                speed: Speed::Slowest
            },
            parse_event(&[Tag::SetEntitySpeedSlowest as u8]).unwrap().1
        );
        assert_eq!(
            Event::EntitySpeed { speed: Speed::Slow },
            parse_event(&[Tag::SetEntitySpeedSlow as u8]).unwrap().1
        );
        assert_eq!(
            Event::EntitySpeed {
                speed: Speed::Normal
            },
            parse_event(&[Tag::SetEntitySpeedNormal as u8]).unwrap().1
        );
        assert_eq!(
            Event::EntitySpeed { speed: Speed::Fast },
            parse_event(&[Tag::SetEntitySpeedFast as u8]).unwrap().1
        );
        assert_eq!(
            Event::EntitySpeed {
                speed: Speed::Fastest
            },
            parse_event(&[Tag::SetEntitySpeedFastest as u8]).unwrap().1
        );

        assert_eq!(
            Event::FadeScreen,
            parse_event(&[Tag::FadeScreen as u8]).unwrap().1
        );

        assert_eq!(
            Event::HideCharacter,
            parse_event(&[Tag::HideCharacter as u8]).unwrap().1
        );

        assert_eq!(
            Event::HideMiniMap,
            parse_event(&[Tag::HideMiniMap as u8]).unwrap().1
        );

        assert_eq!(
            Event::LoadMap {
                map: 0x603,
                x: 8,
                y: 8,
                mode: 0,
                variant: Tag::LoadMap as u8
            },
            parse_event(&[Tag::LoadMap as u8, 0x03, 0x06, 0x08, 0x08, 0x00])
                .unwrap()
                .1
        );

        assert_eq!(
            Event::LoadMap {
                map: 0x603,
                x: 8,
                y: 8,
                mode: 0,
                variant: Tag::LoadMap2 as u8
            },
            parse_event(&[Tag::LoadMap2 as u8, 0x03, 0x06, 0x08, 0x08, 0x00])
                .unwrap()
                .1
        );

        assert_eq!(
            Event::HideCharacter,
            parse_event(&[Tag::HideCharacter as u8]).unwrap().1
        );

        assert_eq!(
            Event::Pause { frames: 10 },
            parse_event(&[Tag::Pause as u8, 10]).unwrap().1
        );

        assert_eq!(
            Event::ShowCharacter,
            parse_event(&[Tag::ShowCharacter as u8]).unwrap().1
        );

        assert_eq!(
            Event::ShowMiniMap,
            parse_event(&[Tag::ShowMiniMap as u8]).unwrap().1
        );

        assert_eq!(
            Event::TurnCharacter { dir: Direction::Up },
            parse_event(&[Tag::TurnCharacterUp as u8]).unwrap().1
        );

        assert_eq!(
            Event::TurnCharacter {
                dir: Direction::Right
            },
            parse_event(&[Tag::TurnCharacterRight as u8]).unwrap().1
        );

        assert_eq!(
            Event::TurnCharacter {
                dir: Direction::Down
            },
            parse_event(&[Tag::TurnCharacterDown as u8]).unwrap().1
        );

        assert_eq!(
            Event::TurnCharacter {
                dir: Direction::Left
            },
            parse_event(&[Tag::TurnCharacterLeft as u8]).unwrap().1
        );

        assert_eq!(
            Event::UnfadeScreen,
            parse_event(&[Tag::UnfadeScreen as u8]).unwrap().1
        );

        assert_eq!(
            Event::UnknownCmdC7 { args: [0xaa, 0x55] },
            parse_event(&[Tag::UnknownCmdC7 as u8, 0xaa, 0x55])
                .unwrap()
                .1
        );
    }

    #[test]
    fn conditional_jump_test() {
        let conditions = [
            Condition {
                is_set: true,
                byte: 0x14,
                bit: 0x4,
            },
            Condition {
                is_set: false,
                byte: 0x82,
                bit: 0x2,
            },
            Condition {
                is_set: true,
                byte: 0x00,
                bit: 0x2,
            },
            Condition {
                is_set: true,
                byte: 0x82,
                bit: 0x0,
            },
            Condition {
                is_set: true,
                byte: 0x00,
                bit: 0x0,
            },
            Condition {
                is_set: false,
                byte: 0xfff,
                bit: 0x0,
            },
            Condition {
                is_set: false,
                byte: 0x00,
                bit: 0x7,
            },
            Condition {
                is_set: false,
                byte: 0x00,
                bit: 0x0,
            },
        ];

        let encoded = [
            [0xa4, 0x80],
            [0x12, 0x04],
            [0x02, 0x80],
            [0x10, 0x84],
            [0x00, 0x80],
            [0xf8, 0x7f],
            [0x07, 0x00],
            [0x00, 0x00],
        ];
        for i in 0..8 {
            let mut c_vec = Vec::new();
            let mut bytes = vec![Tag::ConditionJumpAnd1 as u8 + i];
            for c in 0..=i {
                c_vec.push(conditions[c as usize].clone());
            }
            for c in 0..=i {
                bytes.push(encoded[c as usize][0]);
                bytes.push(encoded[c as usize][1]);
            }

            bytes.push(0x56);
            bytes.push(0x34);
            bytes.push(0x12);

            assert_eq!(
                Event::ConditionalJump {
                    op: CondOp::And,
                    conditions: c_vec.clone(),
                    addr: 0x123456
                },
                parse_event(&bytes).unwrap().1
            );

            bytes[0] = Tag::ConditionJumpOr1 as u8 + i;
            assert_eq!(
                Event::ConditionalJump {
                    op: CondOp::Or,
                    conditions: c_vec,
                    addr: 0x123456
                },
                parse_event(&bytes).unwrap().1
            );
        }
    }

    #[test]
    fn move_diagonal_test() {
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::RightUp,
                steps: [1, 1]
            },
            parse_event(&[Tag::MoveDiagRightUp1x1 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::RightUp,
                steps: [1, 2]
            },
            parse_event(&[Tag::MoveDiagRightUp1x2 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::RightUp,
                steps: [2, 1]
            },
            parse_event(&[Tag::MoveDiagRightUp2x1 as u8]).unwrap().1
        );

        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::RightDown,
                steps: [1, 1]
            },
            parse_event(&[Tag::MoveDiagRightDown1x1 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::RightDown,
                steps: [1, 2]
            },
            parse_event(&[Tag::MoveDiagRightDown1x2 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::RightDown,
                steps: [2, 1]
            },
            parse_event(&[Tag::MoveDiagRightDown2x1 as u8]).unwrap().1
        );

        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::LeftUp,
                steps: [1, 1]
            },
            parse_event(&[Tag::MoveDiagLeftUp1x1 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::LeftUp,
                steps: [1, 2]
            },
            parse_event(&[Tag::MoveDiagLeftUp1x2 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::LeftUp,
                steps: [2, 1]
            },
            parse_event(&[Tag::MoveDiagLeftUp2x1 as u8]).unwrap().1
        );

        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::LeftDown,
                steps: [1, 1]
            },
            parse_event(&[Tag::MoveDiagLeftDown1x1 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::LeftDown,
                steps: [1, 2]
            },
            parse_event(&[Tag::MoveDiagLeftDown1x2 as u8]).unwrap().1
        );
        assert_eq!(
            Event::MoveDiag {
                dir: Diagonal::LeftDown,
                steps: [2, 1]
            },
            parse_event(&[Tag::MoveDiagLeftDown2x1 as u8]).unwrap().1
        );
    }

    #[test]
    fn parse_graphical_action_test() {
        for i in 0..0x40 {
            assert_eq!(
                Event::GraphicalAction {
                    action: i,
                    flipped: false
                },
                parse_event(&[i as u8] as &[u8]).unwrap().1
            );

            assert_eq!(
                Event::GraphicalAction {
                    action: i,
                    flipped: true
                },
                parse_event(&[(i | 0x40) as u8] as &[u8]).unwrap().1
            );
        }
    }

    #[test]
    fn parse_movement_test() {
        for dir in 0..=0x3 {
            for steps in 0..=0x7 {
                let cmd: u8 = 0x80 | steps << 2 | dir;
                assert_eq!(
                    Event::Move {
                        dir: dir,
                        steps: steps
                    },
                    parse_event(&[cmd] as &[u8]).unwrap().1
                );
            }
        }
    }
}

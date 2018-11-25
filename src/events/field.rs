use events::action;
use nom::{le_u16, le_u24, le_u8, ErrorKind};

#[derive(Debug, PartialEq)]
enum Tag {
    // Tags 0x00 = 0x34 are listed in ActionQueue
    MakeChar0Lead = 0x47,

    DialogWait = 0x49,
    DispTextBoxWait = 0x4B,

    InvokeBattle = 0x4e,

    InvokeBattleOnChestOpen = 0x8e,

    UnfadeScreen = 0x96,

    Call = 0xb2,
    JumpIfBattleSwitch = 0xb7,

    BranchIfEventBit = 0xc0,

    Ret = 0xfe,
    Nop = 0xff,
}

// These also correspond to their Event Tags.
pub enum ActionQueue {
    FirstPartyMember = 0x31,
    SecondPartyMember = 0x32,
    ThirdPartyMember = 0x33,
    FourthPartyMember = 0x34,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    ActionQueue {
        queue_id: u8, // ToDo Enum?
        actions: Vec<action::Action>,
        wait: u8,
        len: u8,
    },
    BranchIfEventBit {
        bit: u16,
        addr: u32,
    },
    Call {
        addr: u32,
    },
    /*
    ClearEventBit {
       bit: u16,
    },
    */
    Dialog {
        msg: u16,
        wait: bool,
    },
    DialogWait,
    InvokeBattle,
    InvokeBattleOnChestOpen,
    JumpIfBattleSwitch {
        switch: u8,
        addr: u32,
    },
    MakeChar0Lead,
    Nop,

    UnfadeScreen,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Script {
    events: Vec<Event>,
}

macro_rules! parse_action_queue {
    ($i:expr, $tag:expr) => {
        do_parse!(
            $i,
            tag!(&[$tag as u8])
                >> info: take!(1)
                >> actions: many_till!(action::parse_action, tag!(&[0xff]))
                >> (Event::ActionQueue {
                    queue_id: $tag as u8,
                    actions: actions.0,
                    wait: info[0] & 0x80,
                    len: info[0] & 0x7f
                })
        )
    };
}

named!(parse_branch_if_event_bit<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::BranchIfEventBit as u8]) >>
        bit: le_u16 >>
        addr: le_u24 >>
        (Event::BranchIfEventBit {
            bit: bit,
            addr: addr,
            })));

named!(parse_call<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::Call as u8]) >>
        addr: le_u24 >>
        (Event::Call{
            addr: addr
            })));

//named!(parse_clear_event_bit3<&[u8], Event>,
//    do_parse!(
//        tag!(&[EventTag::ClearEventBit3 as u8]) >>
//        v: take!(1) >>
//        (Event::ClearEventBit{
//            bit: 0x300 | v[0] as u16
//            })));

named!(parse_disp_text_box_wait<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::DispTextBoxWait as u8]) >>
        msg: le_u16 >>
        (Event::Dialog{
            msg: msg,
            wait: true,
            })));

named!(parse_jump_if_battle_switch<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::JumpIfBattleSwitch as u8]) >>
        s: le_u8 >>
        addr: le_u24 >>
        (Event::JumpIfBattleSwitch{
            switch: s,
            addr: addr,
            })));

named!(pub parse_event<&[u8], Event>, alt!(
    parse_action_queue!(ActionQueue::FirstPartyMember) |
    parse_action_queue!(ActionQueue::SecondPartyMember) |
    parse_action_queue!(ActionQueue::ThirdPartyMember) |
    parse_action_queue!(ActionQueue::FourthPartyMember) |

    parse_simple_event!(Tag::DialogWait, Event::DialogWait) |
    parse_simple_event!(Tag::InvokeBattle, Event::InvokeBattle) |
    parse_simple_event!(Tag::InvokeBattleOnChestOpen,
        Event::InvokeBattleOnChestOpen) |
    parse_simple_event!(Tag::MakeChar0Lead, Event::MakeChar0Lead) |
    parse_simple_event!(Tag::Nop, Event::Nop) |
    parse_simple_event!(Tag::UnfadeScreen, Event::UnfadeScreen) |

    parse_branch_if_event_bit |
    parse_call |
    parse_disp_text_box_wait |
    parse_jump_if_battle_switch |

    // This is kinda a hack to force an error on unrecognized tags.  We know
    // parse_nop will not match because it's listed above.  We'd never get here
    // if the tag was a nop....
       return_error!(ErrorKind::Custom(42),
     parse_simple_event!(Tag::Nop, Event::Nop))
));

named!(pub parse_script<&[u8], Script>, do_parse!(
    events: many_till!(parse_event, tag!(&[Tag::Ret as u8]) ) >>
    (Script{events: events.0})));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_events_test() {
        assert_eq!(
            Event::BranchIfEventBit {
                bit: 0x127,
                addr: 0x108
            },
            parse_event(&[Tag::BranchIfEventBit as u8, 0x27, 0x01, 0x08, 0x01, 0x00])
                .unwrap()
                .1
        );

        assert_eq!(
            Event::Call { addr: 0x00 },
            parse_event(&[Tag::Call as u8, 0x00, 0x00, 0x00]).unwrap().1
        );
        assert_eq!(
            Event::Call { addr: 0x5e33 },
            parse_event(&[Tag::Call as u8, 0x33, 0x5e, 0x00]).unwrap().1
        );

        assert_eq!(
            Event::Dialog {
                msg: 0x0b85,
                wait: true
            },
            parse_event(&[Tag::DispTextBoxWait as u8, 0x85, 0x0b])
                .unwrap()
                .1
        );
        assert_eq!(
            Event::DialogWait,
            parse_event(&[Tag::DialogWait as u8]).unwrap().1
        );

        assert_eq!(
            Event::InvokeBattle,
            parse_event(&[Tag::InvokeBattle as u8]).unwrap().1
        );
        assert_eq!(
            Event::InvokeBattleOnChestOpen,
            parse_event(&[Tag::InvokeBattleOnChestOpen as u8])
                .unwrap()
                .1
        );

        assert_eq!(
            Event::JumpIfBattleSwitch {
                switch: 0x40,
                addr: 0x32
            },
            parse_event(&[Tag::JumpIfBattleSwitch as u8, 0x40, 0x32, 0x00, 0x00])
                .unwrap()
                .1
        );

        assert_eq!(
            Event::MakeChar0Lead,
            parse_event(&[Tag::MakeChar0Lead as u8]).unwrap().1
        );

        assert_eq!(Event::Nop, parse_event(&[Tag::Nop as u8]).unwrap().1);

        assert_eq!(
            Event::UnfadeScreen,
            parse_event(&[Tag::UnfadeScreen as u8]).unwrap().1
        );
    }

    #[test]
    fn script_test() {
        assert_eq!(
            Script {
                events: vec![Event::Dialog {
                    msg: 0x0b85,
                    wait: true
                }]
            },
            parse_script(&[0x4b, 0x85, 0x0b, 0xfe]).unwrap().1
        )
    }
}

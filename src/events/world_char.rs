use nom::{le_u16, le_u8, ErrorKind};

#[derive(Debug, PartialEq)]
enum Tag {
    LoadMap = 0xd2,

    FadeScreen = 0xd9,

    Pause = 0xe0,

    Ret = 0xfe,
    End = 0xff,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    FadeScreen,
    LoadMap { map: u16, x: u8, y: u8, mode: u8 },
    Pause { frames: u8 },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Script {
    events: Vec<Event>,
}

// TODO: this could common across all event types
macro_rules! parse_simple_event {
    ($i:expr, $tag:expr, $t:expr) => {
        map!($i, tag!(&[$tag as u8]), |_| $t)
    };
}

named!(parse_load_map<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::LoadMap as u8]) >>
        map: le_u16 >>
        x: le_u8 >>
        y: le_u8 >>
        mode: le_u8 >>
        (Event::LoadMap{map: map, x: x, y: y, mode: mode})
            ));

named!(parse_pause<&[u8], Event>,
    do_parse!(
        tag!(&[Tag::Pause as u8]) >>
        frames: le_u8 >>
        (Event::Pause{frames: frames})
            ));

named!(pub parse_event<&[u8], Event>, alt!(
    parse_simple_event!(Tag::FadeScreen, Event::FadeScreen) |

    parse_load_map |
    parse_pause |

    return_error!(ErrorKind::Custom(42),
        parse_simple_event!(Tag::FadeScreen, Event::FadeScreen))
));

named!(pub parse_script<&[u8], Script>, do_parse!(
    events: many_till!(parse_event,
    alt!(tag!(&[Tag::End as u8]) | tag!(&[Tag::Ret as u8])) ) >>
    (Script{events: events.0})));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_events_test() {
        assert_eq!(
            Event::FadeScreen,
            parse_event(&[Tag::FadeScreen as u8]).unwrap().1
        );
    }
}

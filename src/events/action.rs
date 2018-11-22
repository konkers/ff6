#[derive(Debug, PartialEq)]
enum Tag {
    CenterOnScreen = 0xd7,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    CenterOnScreen,
}

macro_rules! parse_simple_action {
    ($i:expr, $tag:expr, $t:expr) => {
        map!($i, tag!(&[$tag as u8]), |_| $t)
    };
}

named!(pub parse_action<&[u8], Action>,
 parse_simple_action!(Tag::CenterOnScreen, Action::CenterOnScreen));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_events_test() {
        assert_eq!(
            Action::CenterOnScreen,
            parse_action(&[Tag::CenterOnScreen as u8]).unwrap().1
        );
    }
}

#![macro_use]

#[macro_export]
macro_rules! parse_simple_event {
    ($i:expr, $tag:expr, $t:expr) => {
        map!($i, tag!(&[$tag as u8]), |_| $t)
    };
}

#[macro_export]
macro_rules! parse_range {
    ($i:expr, $min:expr, $max:expr) => {{
        use nom::lib::std::result::Result::*;
        use nom::{need_more, InputTake};
        use nom::{ErrorKind, IResult, Needed};

        // Expand to i32 so we don't run against the ends of u8.
        let min = $min as i32;
        let max = $max as i32;
        let res: IResult<_, _>;
        if $i.len() < 1 {
            res = need_more($i, Needed::Size(1));
        } else if min <= ($i[0] as i32) && ($i[0] as i32) <= max {
            res = Ok($i.take_split(1))
        } else {
            res = Err(nom::Err::Error(nom::Context::Code($i, ErrorKind::Tag)))
        }
        res
    };};
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_range_test() {
        assert_eq!(
            (&[2u8, 3u8] as &[u8], &[0x0u8] as &[u8]),
            parse_range!(&[0x0u8, 2, 3] as &[u8], 0, 2).unwrap()
        );
        assert_eq!(
            (&[2u8, 3u8] as &[u8], &[0x1u8] as &[u8]),
            parse_range!(&[0x1u8, 2, 3] as &[u8], 0, 2).unwrap()
        );

        use nom::{Context, Err, ErrorKind, Needed};
        assert_eq!(
            Err(Err::Error(Context::Code(
                &[3u8, 2, 3] as &[u8],
                ErrorKind::Tag
            ))),
            parse_range!(&[0x3u8, 2, 3] as &[u8], 0, 2)
        );
        assert_eq!(
            Err(Err::Incomplete(Needed::Size(1))),
            parse_range!(&[] as &[u8], 0, 2)
        );
    }
}

use nom::{error_to_list, ErrorKind};
use std::collections::HashMap;
use std::error;
use std::error::Error;
use std::fmt;

mod action;
mod field;
mod world_char;

#[derive(Debug)]
pub enum Script {
    FieldScript(field::Script),
    WorldCharScript(world_char::Script),
}

#[derive(Debug, Clone)]
pub struct ParseError {
    tag: u8,
    offset: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Unknown event tag 0x{:x} at 0x{:06x}",
            self.tag, self.offset
        ))
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "ParseError"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn err_text(data: &[u8], e: &nom::Err<&[u8]>) -> String {
    if let nom::Err::Failure(f) = &e {
        let errors = error_to_list(&f);
        match &errors[..] {
            [(_, ErrorKind::Custom(42)), (_, ErrorKind::Tag)] => format!(
                "Parse error: unknown tag 0x{:02x} and 0x{:06x}",
                errors[0].0[0],
                nom::slice_to_offsets(data, errors[0].0).0,
            ),
            _ => {
                for e in errors {
                    println!("{:?} {}", e.1, e.0[0]);
                }
                format!("Parse Error")
            }
        }
    } else {
        format!("Parse Error")
    }
}

pub fn parse(data: &[u8]) -> Result<HashMap<usize, Script>, Box<Error>> {
    let mut d = data;
    let mut scripts = HashMap::new();
    let mut err = String::from("");

    while d.len() > 0 {
        err = String::from("");

        let res = world_char::parse_script(d);
        match res {
            Ok(event) => {
                d = event.0;
                scripts.insert(
                    nom::slice_to_offsets(data, d).0,
                    Script::WorldCharScript(event.1),
                );
                continue;
            }
            Err(e) => {
                err.push_str(&format!("world_char {}\n", err_text(data, &e)));
            }
        };

        let res = field::parse_script(d);
        match res {
            Ok(event) => {
                d = event.0;
                scripts.insert(
                    nom::slice_to_offsets(data, d).0,
                    Script::FieldScript(event.1),
                );
                continue;
            }
            Err(e) => {
                err.push_str(&format!("field {}\n", err_text(data, &e)));
            }
        };

        break;
    }

    if d.len() > 0 {
        println!("{} bytes left.", d.len());
        println!("Errors: {}", err);
    }

    Ok(scripts)
}

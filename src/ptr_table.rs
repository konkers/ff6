use simple_error::SimpleError;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub addr: u32,
    pub len: usize,
}
impl Entry {
    pub fn slice<'a>(&self, data: &'a [u8]) -> Result<&'a [u8], Box<Error>> {
        let start = self.addr as usize;
        let end = start + self.len;
        if data.len() < end {
            Err(SimpleError::new(format!(
                "data needs to be at least {} bytes long.  Is {}.",
                end,
                data.len()
            ))
            .into())
        } else {
            Ok(&data[start..end])
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub entries: Vec<Entry>,
}

fn decode_u16(data: &[u8]) -> u16 {
    data[0] as u16 | ((data[1] as u16) << 8)
}

impl Table {
    pub fn new(data: &[u8], entries: usize, offset: u32) -> Table {
        let mut table = Table {
            entries: Vec::new(),
        };
        // The last entry exists only for sizing the previous range.
        for i in 0..(entries - 1) {
            let addr = decode_u16(&data[i * 2..]);
            let next_addr = decode_u16(&data[(i + 1) * 2..]);
            let len = next_addr - addr;
            table.entries.push(Entry {
                addr: offset + addr as u32,
                len: len as usize,
            });
        }

        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(
            Table {
                entries: vec!(
                    Entry {
                        addr: 0x12345002,
                        len: 2
                    },
                    Entry {
                        addr: 0x12345004,
                        len: 0x100
                    },
                    Entry {
                        addr: 0x12345104,
                        len: 0x1efc
                    },
                )
            },
            Table::new(
                &[0x02, 0x00, 0x04, 0x00, 0x04, 0x01, 0x00, 0x20,],
                4,
                0x12345000
            )
        );
    }
}

use log::*;

use crate::wire::*;


/// A **TXT** record, which holds arbitrary descriptive text.
///
/// # Encoding
///
/// The text encoding is not specified, but this crate treats it as UTF-8.
/// Invalid bytes are turned into the replacement character.
///
/// # References
///
/// - [RFC 1035 §3.3.14](https://tools.ietf.org/html/rfc1035) — Domain Names, Implementation and Specification (November 1987)
#[derive(PartialEq, Debug)]
pub struct TXT {

    /// The message contained in the record.
    pub message: String,
}

impl Wire for TXT {
    const NAME: &'static str = "TXT";
    const RR_TYPE: u16 = 16;

    #[cfg_attr(all(test, feature = "with_mutagen"), ::mutagen::mutate)]
    fn read(len: u16, c: &mut Cursor<&[u8]>) -> Result<Self, WireError> {
        let mut buf = Vec::new();
        let mut total_len = 0_u16;

        loop {
            let next_len = c.read_u8()?;
            total_len += u16::from(next_len) + 1;
            trace!("Parsed slice length -> {:?} (total so far {:?})", next_len, total_len);

            for _ in 0 .. next_len {
                buf.push(c.read_u8()?);
            }

            if next_len < 255 {
                break;
            }
            else {
                trace!("Got length 255, so looping");
            }
        }

        if len == total_len {
            debug!("Length matches expected");
        }
        else {
            warn!("Expected length {} but read {} bytes", len, buf.len());
        }

        let message = String::from_utf8_lossy(&buf).to_string();
        trace!("Parsed message -> {:?}", message);

        if len == total_len {
            trace!("Length is correct");
            Ok(Self { message })
        }
        else {
            warn!("Length is incorrect (record length {:?}, message length {:?})", len, total_len);
            Err(WireError::WrongLabelLength { expected: len, got: total_len })
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_one_iteration() {
        let buf = &[
            0x06,  // message chunk length
            0x74, 0x78, 0x74, 0x20, 0x6d, 0x65,  // message chunk
        ];

        assert_eq!(TXT::read(buf.len() as _, &mut Cursor::new(buf)).unwrap(),
                   TXT {
                       message: String::from("txt me"),
                   });
    }

    #[test]
    fn parses_two_iterations() {
        let buf = &[
            0xFF,  // message chunk length
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41,  // exactly two hundred and fifty five ‘A’s (screaming)
            0x04,  // message chunk length
            0x41, 0x41, 0x41, 0x41,  // four more ‘A’s (the scream abruptly stops)
        ];

        assert_eq!(TXT::read(buf.len() as _, &mut Cursor::new(buf)).unwrap(),
                   TXT {
                       message: String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAA"),
                   });
        // did you know you can just _write_ code like this, and nobody will stop you?
    }

    #[test]
    fn record_empty() {
        assert_eq!(TXT::read(0, &mut Cursor::new(&[])),
                   Err(WireError::IO));
    }

    #[test]
    fn buffer_ends_abruptly() {
        let buf = &[
            0x06, 0x74,  // the start of a message
        ];

        assert_eq!(TXT::read(23, &mut Cursor::new(buf)),
                   Err(WireError::IO));
    }
}

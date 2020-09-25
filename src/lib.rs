use std::io::{Read, Result as IoResult,};
use std::iter::Iterator;

use byte_string::ByteString;

const NEW_LINE: u8 = b'\n';
const CARRIAGE_RETURN: u8 = b'\r';

pub type ByteLine = ByteString;
pub type ByteLineResult = IoResult<ByteLine>;

#[derive(Debug, Copy, Clone)]
pub struct ByteLines<B> {
  buf: B,
}

impl<B: Read> Iterator for ByteLines<B> {
  type Item = ByteLineResult;

  fn next(&mut self) -> Option<ByteLineResult> {
    let mut buf = vec![];
    let bytes = self.buf.by_ref().bytes();

    for byte in bytes {
      if let Ok(byte) = byte {
        buf.push(byte);

        if is_newline(byte) {
          break;
        }
      }
    }

    if buf.is_empty() {
       return None;
    }

    let line = ByteLine::new(buf);
    Some(Ok(line))
  }
}

pub trait ReadByteLines<T> {
  fn byte_lines(self: Self) -> ByteLines<T>;
}

impl<T> ReadByteLines<T> for T {
  fn byte_lines(self: T) -> ByteLines<T> {
    ByteLines { buf: self }
  }
}

fn is_newline(chr: u8) -> bool {
  chr == NEW_LINE || chr == CARRIAGE_RETURN
}

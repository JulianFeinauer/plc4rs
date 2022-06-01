use std::io::Read;
use crate::Endianess;

pub trait ReadBuffer {
    fn read_u8(&mut self) -> u8;
    fn read_u16(&mut self) -> u16;
    fn read_bytes(&mut self, length: usize) -> Vec<u8>;
}

pub struct _ReadBuffer<'a, T: Read> {
    position: u64,
    endianness: Endianess,
    reader: &'a mut T,
}

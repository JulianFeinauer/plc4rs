use std::io::Write;
use std::marker::PhantomData;

use crate::read_buffer::ReadBuffer;
use crate::write_buffer::WriteBuffer;

mod write_buffer;
mod modbus;
mod read_buffer;

#[allow(dead_code)]
enum Endianess {
    LittleEndian,
    BigEndian
}

trait Message {

    fn serialize(&self, writer: &mut Box<dyn WriteBuffer>) -> Result<(), ()>;
    fn deserialize(&self, reader: &mut Box<dyn ReadBuffer>) -> Box<dyn Message>;

}

#[cfg(test)]
mod tests {

}

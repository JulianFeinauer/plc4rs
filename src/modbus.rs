use std::hash::Hasher;
use std::io::{Read, Write};

use crate::Message;
use crate::read_buffer::ReadBuffer;
use crate::write_buffer::WriteBuffer;

// [type ModbusPDUWriteFileRecordResponseItem
//     [simple     uint 8     referenceType]
//     [simple     uint 16    fileNumber]
//     [simple     uint 16    recordNumber]
//     [implicit   uint 16    recordLength   'COUNT(recordData) / 2']
//     [array      byte       recordData     length  'recordLength']
// ]
struct ModbusPDUWriteFileRecordResponseItem {
    reference_type: u8,
    file_number: u16,
    record_number: u16,
    record_data: Vec<u8>,
}

impl ModbusPDUWriteFileRecordResponseItem {
    fn record_length(&self) -> u16 {
        (self.record_data.len() / 2) as u16
    }
}

impl Message for ModbusPDUWriteFileRecordResponseItem {
    fn serialize(&self, writer: &mut Box<dyn WriteBuffer>) -> Result<(), ()> {
        writer.write_u8(self.reference_type);
        writer.write_u16(self.file_number);
        writer.write_u16(self.record_number);
        writer.write_u16(self.record_length());
        writer.write_bytes(&self.record_data);
        Ok(())
    }

    fn deserialize(&self, reader: &mut Box<dyn ReadBuffer>) -> Box<dyn Message> {
        let reference_type = reader.read_u8();
        let file_number = reader.read_u16();
        let record_number = reader.read_u16();
        let record_length = reader.read_u16();
        let record_data = reader.read_bytes(record_length as usize);

        Box::new(ModbusPDUWriteFileRecordResponseItem {
            reference_type,
            file_number,
            record_number,
            record_data,
        })
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::io::Write;
    use std::rc::Rc;
    use crate::{Endianess, Message};
    use crate::modbus::ModbusPDUWriteFileRecordResponseItem;
    use crate::write_buffer::{InternalWriteBuffer, BitWriter, WriteBuffer};

    #[test]
    fn ser_deser() {
        let message = ModbusPDUWriteFileRecordResponseItem {
            reference_type: 0,
            file_number: 0,
            record_number: 0,
            record_data: vec![1, 2, 3],
        };


        let mut bit_writer = BitWriter {
            position: 0,
            value: 0,
        };

        type A<'a> = dyn Write + 'a;

        let mut bytes: Vec<u8> = vec![];
        let mut a: Box<A> = Box::new(bytes);

        let mut writer: Box<dyn WriteBuffer> = Box::new(InternalWriteBuffer {
            position: 0,
            endianness: Endianess::LittleEndian,
            bit_writer,
            writer:  Box::new(RefCell::new(a)),
        });

        message.serialize(&mut writer);
    }
}

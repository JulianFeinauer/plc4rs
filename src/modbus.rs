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
#[derive(PartialEq,Eq,Clone,Debug)]
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
    type M = ModbusPDUWriteFileRecordResponseItem;

    fn serialize<T: Write>(&self, writer: &mut WriteBuffer<T>) -> Result<(), std::io::Error> {
        writer.write_u8(self.reference_type)?;
        writer.write_u16(self.file_number)?;
        writer.write_u16(self.record_number)?;
        writer.write_u16(self.record_length())?;
        writer.write_bytes(&self.record_data)?;
        Ok(())
    }

    fn deserialize<T: Read>(&self, reader: &mut ReadBuffer<T>) -> Result<Self::M, std::io::Error> {
        let reference_type = reader.read_u8()?;
        let file_number = reader.read_u16()?;
        let record_number = reader.read_u16()?;
        let record_length = reader.read_u16()?;
        let record_data = reader.read_bytes(2 * record_length as usize)?;

        Ok(ModbusPDUWriteFileRecordResponseItem {
            reference_type,
            file_number,
            record_number,
            record_data,
        })
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use crate::{Endianess, Message, ReadBuffer};
    use crate::modbus::ModbusPDUWriteFileRecordResponseItem;
    use crate::write_buffer::{WriteBuffer};

    #[test]
    fn ser_deser() {
        let message = ModbusPDUWriteFileRecordResponseItem {
            reference_type: 0,
            file_number: 0,
            record_number: 0,
            record_data: vec![1, 2, 3, 4],
        };

        let bytes: Vec<u8> = vec![];

        let mut writer = WriteBuffer::new(Endianess::BigEndian, bytes);

        message.serialize(&mut writer);

        let bytes = writer.writer.clone();

        assert_eq!(vec![0, 0, 0, 0, 0, 0, 2, 1, 2, 3, 4], bytes);

        let bytes = writer.writer.clone();
        let mut reader = ReadBuffer::new(Endianess::BigEndian, &*bytes);

        if let Ok(msg) = message.deserialize(&mut reader) {
            assert_eq!(message, msg);
        } else {
            assert!(false);
        }

    }
}

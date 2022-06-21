use crate::packet::Packet;
use crate::{binary_reader, binary_writer};

#[derive(Debug)]
pub struct ServerResponse<'a> {
    pub json: &'a str
}

impl <'a> Packet<'a, super::PacketId> for ServerResponse<'a> {
    fn read(bytes: &'a [u8]) -> anyhow::Result<ServerResponse<'a>> {
        let mut bytes = bytes;

        let packet = ServerResponse {
            json: binary_reader::read_string(&mut bytes)?
        };

        binary_reader::ensure_fully_read(bytes)?;

        Ok(packet)
    }

    fn get_write_size(&self) -> usize {
        5 + self.json.len()
    }

    unsafe fn write<'b>(&self, mut bytes: &'b mut [u8]) -> &'b mut [u8] {
        bytes = binary_writer::write_sized_string(bytes, self.json);
        bytes
    }
}
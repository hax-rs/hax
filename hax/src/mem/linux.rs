use memlib::{MemoryRead, MemoryWrite};

pub struct ExternalMemory;

impl MemoryRead for ExternalMemory {
    fn try_read_bytes_into(&self, _address: u64, _buffer: &mut [u8]) -> Option<()> {
        todo!()
    }
}

impl MemoryWrite for ExternalMemory {
    fn try_write_bytes(&self, _address: u64, _buffer: &[u8]) -> Option<()> {
        todo!()
    }
}

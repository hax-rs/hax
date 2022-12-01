use memlib::{MemoryRead, MemoryWrite};

pub struct ExternalMemory;

impl MemoryRead for ExternalMemory {
    fn try_read_bytes_into(&self, address: u64, buffer: &mut [u8]) -> Option<()> {
        todo!()
    }
}

impl MemoryWrite for ExternalMemory {
    fn try_write_bytes(&self, address: u64, buffer: &[u8]) -> Option<()> {
        todo!()
    }
}

pub struct InternalMemory;

impl MemoryRead for InternalMemory {
    fn try_read_bytes_into(&self, address: u64, buffer: &mut [u8]) -> Option<()> {
        todo!()
    }
}

impl MemoryWrite for InternalMemory {
    fn try_write_bytes(&self, address: u64, buffer: &[u8]) -> Option<()> {
        todo!()
    }
}

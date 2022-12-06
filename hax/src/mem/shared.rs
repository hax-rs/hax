use memlib::MemoryRead;
use memlib::MemoryWrite;

pub struct InternalMemory;

impl MemoryRead for InternalMemory {
    fn try_read_bytes_into(&self, address: u64, buffer: &mut [u8]) -> Option<()> {
        if address == 0 {
            return None;
        }

        unsafe {
            core::ptr::copy_nonoverlapping(address as *const u8, buffer.as_mut_ptr(), buffer.len())
        };

        Some(())
    }
}

impl MemoryWrite for InternalMemory {
    fn try_write_bytes(&self, address: u64, buffer: &[u8]) -> Option<()> {
        if address == 0 {
            return None;
        }

        unsafe {
            core::ptr::copy_nonoverlapping(buffer.as_ptr(), address as *mut u8, buffer.len())
        };

        Some(())
    }
}

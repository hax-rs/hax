use std::mem;

use memlib::{MemoryRead, MemoryWrite};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS;
use windows::Win32::System::Threading::PROCESS_VM_READ;
use windows::Win32::System::Threading::PROCESS_VM_WRITE;

pub struct ExternalMemory {
    handle: HANDLE,
}

impl ExternalMemory {
    pub fn new(pid: i32) -> Self {
        let handle = unsafe {
            OpenProcess(
                PROCESS_ACCESS_RIGHTS(PROCESS_VM_READ.0 | PROCESS_VM_WRITE.0),
                false,
                pid as u32,
            )
        };

        // TODO: Should we return an error here?
        match handle {
            Err(error) => panic!("Failed to open process: {}", error),
            Ok(handle) => Self { handle },
        }
    }
}

impl MemoryRead for ExternalMemory {
    fn try_read_bytes_into(&self, address: u64, buffer: &mut [u8]) -> Option<()> {
        if buffer.len() == 0 {
            return Some(());
        }

        let status = unsafe {
            ReadProcessMemory(
                self.handle,
                address as _,
                buffer.as_mut_ptr() as _,
                mem::size_of_val(buffer) as _,
                None,
            )
        };
        if let Err(error) = status.ok() {
            log::error!("ReadProcessMemory failed: {}", error);
            return None;
        }

        Some(())
    }
}

impl MemoryWrite for ExternalMemory {
    fn try_write_bytes(&self, address: u64, buffer: &[u8]) -> Option<()> {
        if buffer.len() == 0 {
            return Some(());
        }

        let status = unsafe {
            WriteProcessMemory(
                self.handle,
                address as _,
                buffer.as_ptr() as _,
                mem::size_of_val(buffer) as _,
                None,
            )
        };
        if let Err(error) = status.ok() {
            log::error!("WriteProcessMemory failed: {}", error);
            return None;
        }

        Some(())
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader, IoSlice, IoSliceMut},
};

use anyhow::anyhow;
use memlib::{MemoryRead, MemoryWrite};
use nix::{
    sys::uio::{process_vm_readv, process_vm_writev, RemoteIoVec},
    unistd::Pid,
};

#[derive(Clone)]
pub struct ExternalMemory {
    pid: Pid,
}

impl ExternalMemory {
    pub fn new(pid: i32) -> Self {
        Self {
            pid: Pid::from_raw(pid),
        }
    }
}

impl ExternalMemory {
    pub fn base_address(&self, module_name: &str) -> anyhow::Result<usize> {
        let file_name = format!("/proc/{}/maps", self.pid);
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);

        for line in reader.lines().flatten() {
            if line.trim().ends_with(&module_name) {
                let split_line: Vec<&str> = line.split('-').collect();
                let address_str = split_line[0];

                return usize::from_str_radix(address_str, 16)
                    .map_err(|_e| anyhow!("Invalid base address"));
            }
        }

        Err(anyhow!("Failed to find base address"))
    }

    fn read_memory(&self, ptr: usize, buf: &mut [u8]) -> anyhow::Result<()> {
        let remote = [RemoteIoVec {
            base: ptr,
            len: buf.len(),
        }];
        let mut local = [IoSliceMut::new(buf)];
        process_vm_readv(self.pid, &mut local, &remote)?;

        // if result == -1 {
        // todo!()
        // match io::Error::last_os_error().raw_os_error() {
        //     Some(libc::ENOSYS) | Some(libc::EPERM) => {
        //         // fallback to reading /proc/$pid/mem if kernel does not
        //         // implement process_vm_readv()
        //         let mut procmem = fs::File::open(format!("/proc/{}/mem", self.0))?;
        //         procmem.seek(io::SeekFrom::Start(addr as u64))?;
        //         procmem.read_exact(buf)
        //     }
        //     _ => Err(io::Error::last_os_error()),
        // }
        // } else {
        // Ok(())
        // }

        Ok(())
    }

    fn write_memory(&self, ptr: usize, buf: &[u8]) -> anyhow::Result<()> {
        let local = [IoSlice::new(buf)];
        let remote = [RemoteIoVec {
            base: ptr,
            len: buf.len(),
        }];

        process_vm_writev(self.pid, &local, &remote)?;

        Ok(())
    }
}

impl MemoryRead for ExternalMemory {
    fn try_read_bytes_into(&self, address: u64, buffer: &mut [u8]) -> Option<()> {
        self.read_memory(address as usize, buffer).ok()
    }
}

impl MemoryWrite for ExternalMemory {
    fn try_write_bytes(&self, address: u64, buffer: &[u8]) -> Option<()> {
        self.write_memory(address as usize, buffer).ok()
    }
}

use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref STATIC_VM: Mutex<VM> = Mutex::new(VM::default());
}

pub fn static_vm_mem_store(offset: u32, data_length: u32) -> Result<(), &'static [u8]> {
    STATIC_VM.lock().unwrap().mem_store(offset, data_length)
}

pub fn get_vm() -> VM {
    STATIC_VM.lock().unwrap().clone()
}

#[derive(Debug, Default, Clone)]
pub struct VM {
    // Each page of memory is 64 KiB = 65536 in size.
    // memory_size is an index of the end of mem
    pub memory_size: u32,

    // data exist from init_offset to offset
    pub offset: u32,
    pub init_offset: u32,
}

impl VM {
    pub fn grow_memory(&mut self, number_of_pages: u32) {
        self.memory_size += 65536 * number_of_pages;
    }

    pub fn get_memory_size(&self) -> u32 {
        self.memory_size - self.init_offset
    }

    pub fn mem_store(&mut self, offset: u32, data_length: u32) -> Result<(), &'static [u8]> {
        // TODO: mock offset to prevent randomly allocating.
        let offset = if self.offset > 0 { self.offset } else { offset };

        if data_length == 0 {
            return Ok(());
        }

        if self.memory_size == 0 {
            self.init_offset = offset;
            self.offset = offset;
            self.memory_size = offset;
        }

        let requested_end = offset + data_length;
        if requested_end > self.memory_size {
            self.grow_memory(1)
        }

        if requested_end > self.memory_size {
            panic!("is requested end too large");
        }

        self.offset += data_length;

        Ok(())
    }
}

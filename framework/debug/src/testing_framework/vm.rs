

#[derive(Debug, Default)]
pub struct VM {
    // Each page of memory is 64 KiB = 65536 in size.
    pub memory_size: u32,
}

impl VM {
    pub fn grow_memory(&mut self, number_of_pages: u32) {
        self.memory_size += 65536*number_of_pages;
    }

    pub fn mem_store(&mut self, offset: u32, data_length: u32) -> Result<(), &'static [u8]> {
        if data_length == 0 {
            return Ok(());
        }

        let requested_end  = offset + data_length;
        if requested_end > self.memory_size {
            self.grow_memory(1)
        }

        if requested_end > self.memory_size {
            panic!("is requested end too large");
        }

        Ok(())
    }
}
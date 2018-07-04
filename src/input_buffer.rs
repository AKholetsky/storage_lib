pub struct InputBuffer {
    buffer: String,
}

impl InputBuffer {
    pub fn new(input: &str) -> InputBuffer {
        InputBuffer {
            buffer: String::from(input),
        }
    }

    pub fn trimed_input(input: &str) -> InputBuffer {
        InputBuffer::new(input.trim())
    }

    pub fn print_buffer(&self) -> &str {
        &self.buffer
    }
}

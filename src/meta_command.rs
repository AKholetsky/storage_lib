use input_buffer::InputBuffer;

pub enum MetaCommandResult {
    MetaCommandExit,
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}


pub fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult {
    let result = match input_buffer.print_buffer() {
        ".exit" => MetaCommandResult::MetaCommandExit,
        _ => MetaCommandResult::MetaCommandUnrecognizedCommand
    };
    result
}
use std::io;

struct InputBuffer<'a> {
    buffer: &'a mut String,
    length: usize,
    capacity: usize, // XXX Unused for now.
}

fn print_prompt() {
    use std::io::{self, Write};
    print!("db> ");
    io::stdout().flush().unwrap();
}

fn read_input(input: &mut InputBuffer) {
    match io::stdin().read_line(&mut input.buffer) {
        Ok(bytes_read) => {
            input.length = bytes_read;
        },
        Err(err) => panic!("Error reading input: {}", err),
    };
    // Discard whitespace and trailing newline.
    input.buffer.trim();
}

fn main() {
    loop {
        // Prompt.
        print_prompt();

        // Input.
        // TODO can probably get away initialising this inside of `read_input' and then passing it
        // along back to the caller as an immutable object initially.
        let mut input: InputBuffer = InputBuffer {
            buffer: &mut String::new(),
            length: 0,
            capacity: 0, // XXX Unused for now.
        };
        read_input(&mut input);
        println!("'{}'", input.buffer);
        if input.buffer == ".exit" {
            break;
        }
    }
}

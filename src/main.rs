use std::io;

struct InputBuffer<'a> {
    buffer: &'a String,
    length: usize,
    capacity: usize,
}

fn print_prompt() {
    use std::io::{self, Write};
    print!("db> ");
    io::stdout().flush().unwrap();
}

fn read_input(input: &mut InputBuffer) {
    match io::stdin().read_line(input.buffer) {
        Ok(bytes_read) => {
            input.length = bytes_read;
        },
        Err(err) => panic!("Error reading input: {}", err),
    };
    input.buffer.trim();
}

fn main() {
    loop {
        // Prompt.
        print_prompt();

        // Input.
        let mut input: InputBuffer = InputBuffer {
            buffer: String::new(),
            length: 0,
            capacity: 0,
        };
        read_input(&mut input);
        if input.buffer == ".exit" {
            break;
        }
    }
}

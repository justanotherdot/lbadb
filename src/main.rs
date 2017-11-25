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
            input.buffer.truncate(bytes_read - 1); // XXX Remove trailing newline.
        },
        Err(err) => panic!("Error reading input: {}", err),
    };
}

fn main() {
    loop {
        // Prompt.
        print_prompt();

        // Input.
        let mut input: InputBuffer = InputBuffer {
            buffer: &mut String::new(),
            length: 0,
            capacity: 0, // XXX Unused for now.
        };
        read_input(&mut input);
        if input.buffer == ".exit" {
            break;
        } else {
            println!("Unrecognized command: '{}'.", input.buffer);
        }
    }
}

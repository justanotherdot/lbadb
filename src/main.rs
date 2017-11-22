use std::io;

fn print_prompt() {
    print!("db > ");
}

fn read_input(input: &mut String) {
    io::stdin().read_line(input)
        .expect("Couldn't read line");
}

fn main() {
    loop {
        // Prompt.
        print_prompt();

        // Input.
        let mut input = String::new();
        read_input(&mut input);
        println!("{}", input);
    }
}

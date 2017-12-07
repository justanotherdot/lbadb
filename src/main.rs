use std::io;

#[macro_use]
extern crate nom;
use nom::{IResult,Needed,Err,ErrorKind};

struct InputBuffer<'a> {
    buffer: &'a mut String,
    length: usize,
    // NOTE: Due to the way Rust tracks memory, we don't need a capacity field here.
}

enum MetaCommandResult {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND,
}

enum PrepareResult {
    PREPARE_SUCCESS,
    PREPARE_UNRECOGNIZED_COMMAND,
}

enum StatementType {
    STATEMENT_INSERT,
    STATEMENT_SELECT,
    STATEMENT_INVALID, // Something went wrong setting the type.
}

struct Statement {
    tag: StatementType, // Because `type` is a keyword in Rust.
}

fn print_prompt() {
    use std::io::{self, Write};
    print!("db> ");
    io::stdout().flush().unwrap();
}

fn read_input(input: &mut InputBuffer) {
    match io::stdin().read_line(&mut input.buffer) {
        Ok(bytes_read) => {
            input.length = bytes_read - 1;
            input.buffer.truncate(bytes_read - 1); // XXX Remove trailing newline.
        }
        Err(err) => panic!("Error reading input: {}", err),
    };
}

fn do_meta_command(input: &mut InputBuffer) -> MetaCommandResult {
    use std::process;
    if input.buffer == ".exit" {
        process::exit(0);
    } else {
        MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND
    }
}

// Parser combinators.

// TODO This needs case insensitivity.
named!(insert<&str, &str>, ws!(tag_s!("insert")));

fn prepare_statement(input: &mut InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input.length >= 6 && &input.buffer[0..6] == "insert" {
        statement.tag = StatementType::STATEMENT_INSERT;
        let ir = insert(input.buffer);
        println!("{:?}", ir);
        PrepareResult::PREPARE_SUCCESS
    } else if input.buffer == "select" {
        statement.tag = StatementType::STATEMENT_SELECT;
        PrepareResult::PREPARE_SUCCESS
    } else {
        PrepareResult::PREPARE_UNRECOGNIZED_COMMAND
    }
}

fn execute_statement(statement: &mut Statement) {
    match statement.tag {
        StatementType::STATEMENT_INSERT => {
            println!("This is where we would do an insert.");
        },
        StatementType::STATEMENT_SELECT => {
            println!("This is where we would do a select.");
        },
        StatementType::STATEMENT_INVALID => {
            println!("Something went wrong: A statement wasn't prepared properly and got passed to be executed.");
        }
    }
}

fn main() {
    loop {
        // Prompt.
        print_prompt();

        // Input.
        let mut input: InputBuffer = InputBuffer {
            buffer: &mut String::new(),
            length: 0,
        };
        read_input(&mut input);

        // Meta commands.
        if input.buffer.chars().nth(0).unwrap() == '.' {
            match do_meta_command(&mut input) {
                MetaCommandResult::META_COMMAND_SUCCESS => continue,
                MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND => {
                    println!("Unrecognized command: '{}'.", input.buffer)
                }
            }
        }

        // Statement preparation.
        let mut statement: Statement = Statement {
            tag: StatementType::STATEMENT_INVALID
        };
        match prepare_statement(&mut input, &mut statement) {
            PrepareResult::PREPARE_SUCCESS => {
                // Statement execution.
                execute_statement(&mut statement);
                println!("Executed.");
            },
            PrepareResult::PREPARE_UNRECOGNIZED_COMMAND => {
                println!("Unrecognized command at start of {}", input.buffer);
                break;
            }
        }
    }
}

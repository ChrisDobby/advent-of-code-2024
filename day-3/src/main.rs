use std::fs;

#[derive(Debug)]
enum Command {
    Multiply,
    Do,
    Dont,
}

#[derive(Debug)]
struct State {
    last_char: Option<char>,
    num1: Option<String>,
    num2: Option<String>,
    command: Option<Command>,
    processing: bool,
    total: u32,
}

fn add_last_char(state: State, c: char) -> State {
    State {
        last_char: Some(c),
        num1: None,
        num2: None,
        command: None,
        ..state
    }
}

fn clear_state(state: State) -> State {
    State {
        last_char: None,
        num1: None,
        num2: None,
        command: None,
        ..state
    }
}

fn set_command(state: State, command: Command, c: char) -> State {
    State {
        last_char: Some(c),
        num1: None,
        num2: None,
        command: Some(command),
        ..state
    }
}

fn handle_open(state: State, c: char) -> State {
    match state.last_char {
        Some('l') => State {
            last_char: Some(c),
            num1: Some("".to_owned()),
            num2: None,
            ..state
        },
        Some('o') => State {
            last_char: Some(c),
            num2: None,
            ..state
        },
        Some('t') => State {
            last_char: Some(c),
            num2: None,
            ..state
        },
        _ => State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            ..state
        },
    }
}

fn handle_close(state: State, allow_disabled: bool) -> State {
    match state.command {
        Some(Command::Multiply) => {
            if state.processing && state.num1 != None && state.num2 != None {
                State {
                    last_char: None,
                    num1: None,
                    num2: None,
                    command: None,
                    processing: state.processing,
                    total: state.total
                        + state.num1.unwrap().parse::<u32>().unwrap()
                            * state.num2.unwrap().parse::<u32>().unwrap(),
                }
            } else {
                State {
                    last_char: None,
                    num1: None,
                    num2: None,
                    command: None,
                    ..state
                }
            }
        }
        Some(Command::Do) => State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            processing: true,
            ..state
        },
        Some(Command::Dont) => State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            processing: !allow_disabled,
            ..state
        },
        _ => State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            ..state
        },
    }
}

fn handle_comma(state: State, c: char) -> State {
    match state.last_char {
        Some(',') => State {
            last_char: None,
            num1: None,
            num2: None,
            ..state
        },
        _ => {
            if state.num1 == None {
                State {
                    last_char: None,
                    num1: None,
                    num2: None,
                    ..state
                }
            } else {
                State {
                    last_char: Some(c),
                    num2: Some("".to_owned()),
                    ..state
                }
            }
        }
    }
}

fn handle_number(state: State, c: char) -> State {
    if state.num2 == None && state.num1 != None {
        State {
            num1: Some(state.num1.unwrap() + &c.to_string()),
            ..state
        }
    } else if state.num2 != None {
        State {
            num2: Some(state.num2.unwrap() + &c.to_string()),
            ..state
        }
    } else {
        State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            ..state
        }
    }
}

fn scan_memory(state: State, allow_disabled: bool, c: char) -> State {
    match c {
        'm' => match state.last_char {
            None => add_last_char(state, c),
            _ => clear_state(state),
        },
        'u' => match state.last_char {
            Some('m') => add_last_char(state, c),
            _ => clear_state(state),
        },
        'l' => match state.last_char {
            Some('u') => set_command(state, Command::Multiply, c),
            _ => clear_state(state),
        },
        'd' => add_last_char(state, c),
        'o' => match state.last_char {
            Some('d') => set_command(state, Command::Do, c),
            _ => clear_state(state),
        },
        'n' => match state.last_char {
            Some('o') => add_last_char(state, c),
            _ => clear_state(state),
        },
        '\'' => match state.last_char {
            Some('n') => add_last_char(state, c),
            _ => clear_state(state),
        },
        't' => match state.last_char {
            Some('\'') => set_command(state, Command::Dont, c),
            _ => clear_state(state),
        },
        '(' => handle_open(state, c),
        ')' => handle_close(state, allow_disabled),
        ',' => handle_comma(state, c),
        '0'..='9' => handle_number(state, c),
        _ => State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            ..state
        },
    }
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let state = content.chars().fold(
        State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            processing: true,
            total: 0,
        },
        |acc, c| scan_memory(acc, false, c),
    );

    let state_with_disabled = content.chars().fold(
        State {
            last_char: None,
            num1: None,
            num2: None,
            command: None,
            processing: true,
            total: 0,
        },
        |acc, c| scan_memory(acc, true, c),
    );

    println!(
        "Total: {} with disabled commands: {}",
        state.total, state_with_disabled.total
    );
}

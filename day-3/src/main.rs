use std::fs;

#[derive(Debug)]
enum Command {
    Multiply,
    Do,
    Dont,
}

fn scan_memory(
    (last_char, num1, num2, command, processing, total): (
        Option<char>,
        Option<String>,
        Option<String>,
        Option<Command>,
        bool,
        u32,
    ),
    allow_disabled: bool,
    c: char,
) -> (
    Option<char>,
    Option<String>,
    Option<String>,
    Option<Command>,
    bool,
    u32,
) {
    match c {
        'm' => match last_char {
            None => (Some(c), None, None, None, processing, total),
            _ => (None, None, None, None, processing, total),
        },
        'u' => match last_char {
            Some('m') => (Some(c), None, None, None, processing, total),
            _ => (None, None, None, None, processing, total),
        },
        'l' => match last_char {
            Some('u') => (
                Some(c),
                None,
                None,
                Some(Command::Multiply),
                processing,
                total,
            ),
            _ => (None, None, None, None, processing, total),
        },
        'd' => (Some(c), None, None, None, processing, total),
        'o' => match last_char {
            Some('d') => (Some(c), None, None, Some(Command::Do), processing, total),
            _ => (None, None, None, None, processing, total),
        },
        'n' => match last_char {
            Some('o') => (Some(c), None, None, None, processing, total),
            _ => (None, None, None, None, processing, total),
        },
        '\'' => match last_char {
            Some('n') => (Some(c), None, None, None, processing, total),
            _ => (None, None, None, None, processing, total),
        },
        't' => match last_char {
            Some('\'') => (Some(c), None, None, Some(Command::Dont), processing, total),
            _ => (None, None, None, None, processing, total),
        },
        '(' => match last_char {
            Some('l') => (
                Some(c),
                Some("".to_owned()),
                None,
                command,
                processing,
                total,
            ),
            Some('o') => (Some(c), num1, None, command, processing, total),
            Some('t') => (Some(c), num1, None, command, processing, total),
            _ => (None, None, None, None, processing, total),
        },
        ')' => match command {
            Some(Command::Multiply) => {
                if processing && num1 != None && num2 != None {
                    (
                        None,
                        None,
                        None,
                        None,
                        processing,
                        total
                            + num1.unwrap().parse::<u32>().unwrap()
                                * num2.unwrap().parse::<u32>().unwrap(),
                    )
                } else {
                    (None, None, None, None, processing, total)
                }
            }
            Some(Command::Do) => (None, None, None, None, true, total),
            Some(Command::Dont) => (None, None, None, None, !allow_disabled, total),

            _ => (None, None, None, None, processing, total),
        },
        ',' => match last_char {
            Some(',') => (None, None, None, command, processing, total),
            _ => {
                if num1 == None {
                    (None, None, None, command, processing, total)
                } else {
                    (
                        Some(c),
                        num1,
                        Some("".to_owned()),
                        command,
                        processing,
                        total,
                    )
                }
            }
        },
        '0'..='9' => {
            if num2 == None && num1 != None {
                (
                    last_char,
                    Some(num1.unwrap() + &c.to_string()),
                    None,
                    command,
                    processing,
                    total,
                )
            } else if num2 != None {
                (
                    last_char,
                    num1,
                    Some(num2.unwrap() + &c.to_string()),
                    command,
                    processing,
                    total,
                )
            } else {
                (None, None, None, None, processing, total)
            }
        }
        _ => (None, None, None, None, processing, total),
    }
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let (_, _, _, _, _, total) = content.chars().fold(
        (
            None::<char>,
            None::<String>,
            None::<String>,
            None::<Command>,
            true,
            0,
        ),
        |acc, c| scan_memory(acc, false, c),
    );

    let (_, _, _, _, _, total_with_disabled) = content.chars().fold(
        (
            None::<char>,
            None::<String>,
            None::<String>,
            None::<Command>,
            true,
            0,
        ),
        |acc, c| scan_memory(acc, true, c),
    );

    println!(
        "Total: {} with disabled commands: {}",
        total, total_with_disabled
    );
}

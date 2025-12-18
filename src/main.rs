mod actions;
mod config;
mod notes;

use std::{env, fmt::Debug};

enum ProgramExitCode {
    BadArgs = -1,
    ConfigError = -2,
    FileError = -3,
    ActionError = -4,
}

enum ActionType {
    NONE,
    LIST,
    CAT,
    EDIT,
    ADD,
    DELETE,
    YANK,
    HELP,
    VERSION,
}

impl std::fmt::Debug for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionType::NONE => f.write_str("NONE"),
            ActionType::LIST => f.write_str("LIST"),
            ActionType::CAT => f.write_str("CAT"),
            ActionType::EDIT => f.write_str("EDIT"),
            ActionType::ADD => f.write_str("ADD"),
            ActionType::DELETE => f.write_str("DELETE"),
            ActionType::YANK => f.write_str("YANK"),
            ActionType::HELP => f.write_str("HELP"),
            ActionType::VERSION => f.write_str("VERSION"),
        }
    }
}

#[derive(Debug)]
struct Arguments {
    pub action: ActionType,
    pub note_id: i32,
    pub note_name: String,
}

fn parse_arguments(raw_args: &Vec<String>) -> Result<Arguments, Box<dyn std::error::Error>> {
    let mut args = Arguments {
        action: ActionType::NONE,
        note_id: -1,
        note_name: String::new(),
    };

    // If there are no arguments perform list action.
    if raw_args.iter().count() == 1 {
        args.action = ActionType::LIST;
        return Ok(args);
    }

    args.note_id = raw_args[1].parse::<i32>().unwrap_or(-1);

    // If first argument is action and not a note.
    if args.note_id == -1 {
        match raw_args[1].as_str() {
            "a" => {
                if raw_args.iter().count() != 3 {
                    return Err("Unkown command")?;
                }
                args.action = ActionType::ADD;
                args.note_name = raw_args[2].clone();
                return Ok(args);
            },
            "l" => args.action = ActionType::LIST,
            "h" => args.action = ActionType::HELP,
            "v" => args.action = ActionType::VERSION,
            _ => return Err("Unkown command")?,
        }
        return Ok(args);
    }

    // If there is a note specified but
    // no action, perform a cat.
    if raw_args.iter().count() == 2 {
        args.action = ActionType::CAT;
        return Ok(args);
    }

    match raw_args[2].as_str() {
        "c" => args.action = ActionType::CAT,
        "e" => args.action = ActionType::EDIT,
        "d" => args.action = ActionType::DELETE,
        "y" => args.action = ActionType::YANK,
        _ => return Err(format!("Command '{}' doesn't exist", raw_args[2]))?,
    }

    Ok(args)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_args: Vec<String> = env::args().collect();

    let args = parse_arguments(&raw_args);
    if args.is_err() {
        eprintln!("\n{}\n", args.unwrap_err());
        std::process::exit(ProgramExitCode::BadArgs as i32);
    }

    let confres = config::get();
    if confres.is_err() {
        eprintln!("\n{}\n", confres.unwrap_err());
        std::process::exit(ProgramExitCode::ConfigError as i32);
    }

    let res = notes::init();
    if res.is_err() {
        eprintln!("\n{}\n", res.unwrap_err());
        std::process::exit(ProgramExitCode::FileError as i32);
    }

    let args_unwraped = args.unwrap();
    let action_type = args_unwraped.action;
    let note_id = args_unwraped.note_id;
    let note_name = args_unwraped.note_name;

    let res: Result<(), Box<dyn std::error::Error>>;

    match action_type {
        ActionType::LIST => res = actions::list(),
        ActionType::CAT => res = actions::cat(note_id),
        ActionType::EDIT => res = actions::edit(note_id),
        ActionType::ADD => res = actions::add(&note_name),
        ActionType::DELETE => res = actions::delete(note_id),
        ActionType::YANK => res = actions::yank(note_id),
        ActionType::HELP => res = actions::help(),
        ActionType::VERSION => res = actions::version(),
        _ => res = Ok(()),
    }

    if res.is_err() {
        eprintln!("\n{}\n", res.unwrap_err());
        std::process::exit(ProgramExitCode::ActionError as i32);
    }

    return Ok(());
}


//! debug CLI command parsing

use errors::*;

use std::str::FromStr;

/// The status of the debug `vm` CLI
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// Debug execution is active
    Success,
    /// Debug execution has ended
    Exit,
}

/// Possible commands for the debug CLI
#[derive(Debug, Clone)]
pub enum Command {
    Run,
    Step(usize),
    Next(usize),
    PrintRegisters,
    PrintStack,
    PrintCode,
    PrintLabels,
    Help,
    Quit,
    Empty,
}

impl Command {
    /// Print Usage description
    pub fn help() {
        let help = [("r, run", "Continue the execution"),
                    ("s, step [NUMBER]", "Step by NUMBER instructions. NUMBER defaults to 1"),
                    ("n, next [NUMBER]", "Show NUMBER instructions. NUMBER defaults to 1"),
                    ("reg, registers", "Print the current value for the registers"),
                    ("st, stack", "Print the current state of the stack"),
                    ("c, code", "Print the code that is beeing run"),
                    ("l, labels", "Print all labels found in the code"),
                    ("h, help", "Print this message"),
                    ("q, quit", "Exit from the debugger")];

        println!();
        println!("COMMANDS:");
        for &(cmd, msg) in &help {
            println!("\t{:20}{}", cmd, msg);
        }
        println!("")
    }
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Command> {
        let mut args = s.split_whitespace();
        if let Some(cmd) = args.next() {
            let res = match cmd.to_lowercase().as_ref() {
                "reg" | "registers" => Ok(Command::PrintRegisters),
                "st" | "stack" => Ok(Command::PrintStack),
                "l" | "labels" => Ok(Command::PrintLabels),
                "c" | "code" => Ok(Command::PrintCode),
                "h" | "help" => Ok(Command::Help),
                "q" | "quit" => Ok(Command::Quit),
                "r" | "run" => Ok(Command::Run),
                "n" | "next" => {
                    Ok(Command::Next(args.next()
                        .unwrap_or("1")
                        .parse()
                        .chain_err(|| "Not a valid argument")?))
                }
                "s" | "step" => {
                    Ok(Command::Step(args.next()
                        .unwrap_or("1")
                        .parse()
                        .chain_err(|| "Not a valid argument")?))
                }
                _ => Err("Command not Found. Try 'help' to find valid commands".into()),
            };

            match args.next() {
                None => res,
                Some(_) => res.and(Err("Invalid argument. See 'help' for usage".into())),
            }

        } else {
            Ok(Command::Empty)
        }
    }
}

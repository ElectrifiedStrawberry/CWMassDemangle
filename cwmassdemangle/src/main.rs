use argh::FromArgs;
use cwdemangle::{demangle, DemangleOptions};

use std::{fs, process::ExitCode};

use crate::argh_cargo::from_env;

mod argh_cargo;

#[derive(FromArgs)]
/// A CodeWarrior C++ symbol demangler.
struct Args {
    /// file name listing all symbols
    #[argh(positional)]
    filename: String,
}

fn main() -> ExitCode {
    let args: Args = from_env();
    let data = fs::read_to_string(args.filename);
    match data {
        Ok(f) => {
            let parts = f.split("\n");
            let opts = &DemangleOptions::default();
            for part in parts {
                let result = demangle(part.trim(), opts);
                match result {
                    Some(name) => println!("{}", ["1", &name].join(" ")),
                    None => println!("{}", ["0", part].join(" ")),
                }
            }
        }
        Err(e) => {
            // For some reason, Unix systems have a really odd thing where
            // its exit codes are only 8 bits. Let's just saturate it there
            // because it's unlikely we'll hit any of the high error codes
            // we'll just saturate it.
            return ExitCode::from(match e.raw_os_error() {
                Some(code) => u8::try_from(code).unwrap_or(u8::MAX),
                None => 1,
            });
        }
    }
    ExitCode::SUCCESS
}

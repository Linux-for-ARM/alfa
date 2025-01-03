//! Functions for console I/O

use anyhow::Result;
use colored::Colorize;
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

pub fn answer<M: Display>(msg: M, def_val: Option<String>) -> Result<String> {
    let mut ans = String::new();

    print!("{}", &msg);
    if let Some(def_val) = &def_val {
        print!(" {}{}{}", "[".dimmed(), def_val.dimmed(), "]".dimmed());
    }
    print!(" {}{} ", "::".bold(), ">".bold().magenta());
    stdout().flush()?;

    stdin().read_line(&mut ans)?;
    ans = ans.trim().to_string();

    if let Some(def_val) = def_val {
        if ans.is_empty() {
            Ok(def_val)
        } else {
            Ok(ans)
        }
    } else {
        Ok(ans)
    }
}

#[macro_export]
macro_rules! yesno {
    ($($arg:tt)*) => {{
        use std::{
            fmt::Display,
            io::{stdout, Write},
        };
        use getch_rs::{Getch, Key, enable_echo_input};
        use colored::Colorize;

        let msg = format!("{}", format_args!($($arg)*));
        print!(" :: {} {} ", msg.bold().magenta(), "[y/n]".dimmed());
        let _ = stdout().flush();

        enable_echo_input();
        let g = Getch::new();

        let rslt = match g.getch() {
            Ok(Key::Char('y')) | Ok(Key::Char('Y')) => true,
            _ => false,
        };

        println!();

        rslt
    }};
}

#[macro_export]
macro_rules! msg {
    () => {
        println!();
    };

    ($($arg:tt)*) => {{
        use colored::Colorize;
        let msg = format!("{}", format_args!($($arg)*));
        println!("==> {}", msg.bold().yellow());
    }};
}

#[macro_export]
macro_rules! process_msg {
    ($($arg:tt)*) => {{
        use ::std::io::{Write, stdout};

        print!("{}", format_args!($($arg)*));
        let _ = stdout().flush();
    }};
}

pub fn process_msg_result(rslt: bool) {
    if rslt {
        println!("{}", "OK".bold().green());
    } else {
        println!("{}", "ERROR".bold().red());
    }
}

pub fn process_msg_result_err<E: Display>(rslt: bool, err: Option<E>) {
    if rslt {
        println!("{}", "OK".bold().green());
    } else {
        match err {
            Some(err) => println!("{}:\t{}", "ERROR".bold().red(), format!("({err})").dimmed()),
            None => println!("{}", "ERROR".bold().red()),
        }
    }
}

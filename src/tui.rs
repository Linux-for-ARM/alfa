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

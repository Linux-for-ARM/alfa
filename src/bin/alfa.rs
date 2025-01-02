//! Master file

use alfa::prepare::Prepare;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

use alfa::config::Config;
use alfa::profile::Profile;

#[derive(Debug, Parser)]
struct Cmd {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate `.config.toml` file
    Config {
        /// Specify the `config.toml` file
        #[arg(short, long, default_value_t = String::from("./.config.toml"))]
        config: String,

        /// Specify the `profile.toml` file
        #[arg(short, long, default_value_t = String::from("./.profile.toml"))]
        profile: String,
    },

    /// Prepare for ALFA build (create user, download files, etc.)
    Prepare {
        /// Specify the `config.toml` file
        #[arg(short, long, default_value_t = String::from("./.config.toml"))]
        config: String,

        /// Specify the `profile.toml` file
        #[arg(short, long, default_value_t = String::from("./.profile.toml"))]
        profile: String,
    },

    /// Build LFA system from source
    Build,

    /// Copy builded files to specified location
    Distcopy {
        /// What to copy
        source: String,

        /// Where to copy
        destination: String,
    },

    /// Clear the system of build files and remove the temporary user
    Sysclean,
}

fn main() -> Result<()> {
    let cmd = Cmd::parse();

    match cmd.command {
        Command::Config { config, profile } => {
            let conf = Config::from_stdin()?;

            print!(
                "\nWrite submited configuration to '{}'... ",
                &config.dimmed()
            );
            match conf.write(&config) {
                Ok(_) => println!("{}", "ok".green()),
                Err(why) => println!("{}:\n\t{}", "error".bold().red(), why),
            }

            print!("Generate build profile to '{}'... ", &profile.dimmed());
            let prof = Profile::new(&conf.system);
            match prof.write(&profile) {
                Ok(_) => println!("{}", "ok".green()),
                Err(why) => println!("{}:\n\t{}", "error".bold().red(), why),
            }
        }
        Command::Prepare { config, profile } => {
            drop(config);

            let profile = Profile::read(&profile)?;
            let prepare = Prepare { profile: &profile };

            println!("Create ALFA dirs...");
            prepare.create_alfa_dirs()?;

            println!("Create temporary build user...");
            prepare.create_user()?;
        }
        _ => todo!(),
    }

    Ok(())
}

//! Preparing host system for ALFA building

use anyhow::Result;
use colored::Colorize;
use std::fs::create_dir_all;
use std::process::Command;

use crate::process_msg;
// use crate::config::Config;
use crate::profile::Profile;
use crate::tui::process_msg_result_err;

pub struct Prepare<'a> {
    pub profile: &'a Profile,
}

impl<'a> Prepare<'a> {
    pub fn create_alfa_dirs(&self) -> Result<()> {
        // create root dir
        process_msg!(
            "Create root directory '{}'",
            &self.profile.build_dir.dimmed()
        );
        let rslt = create_dir_all(&self.profile.build_dir);
        process_msg_result_err(
            rslt.is_ok(),
            if let Err(err) = rslt { Some(err) } else { None },
        );

        // create other dirs
        for i in ["lfa", "src", "scripts"] {
            let dir = format!("{}/{}", &self.profile.build_dir, i);
            process_msg!("Create subdirectory '{}'", &dir.dimmed());
            let rslt = create_dir_all(&dir);
            process_msg_result_err(
                rslt.is_ok(),
                if let Err(err) = rslt { Some(err) } else { None },
            );
        }

        Ok(())
    }

    pub fn create_user(&self) -> Result<()> {
        let grp = Command::new("/sbin/groupadd")
            .arg(&self.profile.user_name)
            .status()?
            .success();
        if !grp {
            return Err(anyhow::Error::msg(format!(
                "Failed to create group '{}'",
                &self.profile.user_name.dimmed()
            )));
        }

        let args = [
            "-s",
            "/bin/bash",
            "-g",
            &self.profile.user_name,
            "-m",
            "-k",
            "/dev/null",
            &self.profile.user_name,
        ];
        let usr = Command::new("/sbin/useradd").args(args).status()?.success();
        if !usr {
            return Err(anyhow::Error::msg(format!(
                "Failed to create user '{}'",
                &self.profile.user_name.dimmed()
            )));
        }

        println!("\nSet password for new user:");
        let pswd = Command::new("/sbin/passwd")
            .arg(&self.profile.user_name)
            .status()?
            .success();
        if !pswd {
            return Err(anyhow::Error::msg(format!(
                "Failed to set password for '{}' user",
                &self.profile.user_name.dimmed()
            )));
        }

        Ok(())
    }
}

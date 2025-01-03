//! (De)serializing the `.profile.toml` file

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use toml;
use uuid::Uuid;

use crate::config::System;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub user_name: String,
    pub build_dir: String,
}

fn str_sanitizer(s: &str) -> String {
    let symbols = [
        '?', '/', ':', '!', '~', '@', '#', '$', '%', '^', '&', '*', '(', ')', '+', '{', '}', '[',
        ']', '\'', '"', '<', '>', '\\', ',', '.',
    ];
    let mut s = String::from(s);
    for sym in symbols {
        s = s.replace(sym, "");
    }
    s = s.replace(' ', "_");
    s
}

impl Profile {
    pub fn new(sys: &System) -> Self {
        let uuid = Uuid::new_v4().simple().to_string();

        Self {
            user_name: format!("lfa_{}", &uuid),
            build_dir: format!(
                "/mnt/{}-{}-{}",
                str_sanitizer(&sys.name),
                str_sanitizer(&sys.version),
                &uuid
            ),
        }
    }

    pub fn to_env_map(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        map.insert("ALFA_USER", self.user_name.clone());
        map.insert("ALFA_BUILD_DIR", self.build_dir.clone());

        map
    }

    pub fn read<P: AsRef<Path>>(pth: P) -> Result<Self> {
        let contents = fs::read_to_string(&pth)?;
        let data = toml::from_str(&contents)?;

        Ok(data)
    }

    pub fn write<P: AsRef<Path>>(&self, pth: P) -> Result<()> {
        let contents = toml::to_string(&self)?;
        fs::write(&pth, contents)?;

        Ok(())
    }
}

//! Packages build instructions

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Instruction {
    pub stage: String,
    pub generic_name: Option<String>,
    pub name: String, // key for `PackageList.package` map, from this we get `version`
    pub file_name: Option<String>,
    pub dir_name: Option<String>,
    pub commands: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

impl Instruction {
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

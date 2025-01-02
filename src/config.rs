//! (De)serializing the `.config.toml` file

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, str::FromStr};
use toml;

use crate::tui::answer;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub system: System,
    pub env_default: EnvDefault,
    pub env: HashMap<String, String>,
}

impl Config {
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

    pub fn from_stdin() -> Result<Self> {
        Ok(Self {
            system: System::from_stdin()?,
            env_default: EnvDefault::from_stdin()?,
            env: {
                let mut envs = HashMap::new();
                let is_set_str = answer(
                    "Set additional environment variables (y/n)?",
                    Some("n".to_string()),
                )?;
                let is_set = &is_set_str == "y" || &is_set_str == "Y";

                if !is_set {
                    envs
                } else {
                    loop {
                        let k = answer("\tname ('end' for exit)", None)?;
                        if &k == "end" {
                            break;
                        }
                        let v = answer(format!("\tvalue of '{}'", &k.dimmed()), None)?;
                        println!(); // отступ в 1 строку чтобы визуально разделить ввод значений переменных

                        envs.insert(k, v);
                    }
                    envs
                }
            },
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub name: String,
    pub version: String,
    pub author: String,
}

impl System {
    pub fn from_stdin() -> Result<Self> {
        println!("{}", "Specify information about ALFA System:".bold());

        Ok(Self {
            name: answer("System name", Some("ALFA".to_string()))?,
            version: answer(
                "System version",
                Some(env!("CARGO_PKG_VERSION").to_string()),
            )?,
            author: answer("System author (builder)", None)?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvDefault {
    pub bits: Bits,
    pub lfa_host: String,
    pub lfa_tgt: String,
    pub lfa_arch: String,
    pub lfa_float: Option<String>,
    pub lfa_fpu: Option<String>,
}

impl EnvDefault {
    pub fn from_stdin() -> Result<Self> {
        println!("\n{}", "Set the default environment variables".bold());

        Ok(Self {
            bits: {
                #[allow(unused_assignments)]
                let mut b = String::new();
                let bits: Bits;

                loop {
                    b = answer("Bits (32/64)", Some("64".to_string()))?;
                    if let Ok(_bits) = Bits::from_str(&b) {
                        bits = _bits;
                        break;
                    }
                }
                bits
            },
            lfa_host: answer("Your host", Some("x86_64-cross-linux-gnu".to_string()))?,
            lfa_tgt: answer("Target", Some("aarch64-linux-musleabihf".to_string()))?,
            lfa_arch: answer("CPU Architecture", Some("armv8.1-a".to_string()))?,
            lfa_float: {
                let float = answer(
                    format!("Float type ({})", "FOR ALL BUT ARM-V8 ARCHITECTURES".bold()),
                    Some("".to_string()),
                )?;
                if float.is_empty() {
                    None
                } else {
                    Some(float)
                }
            },
            lfa_fpu: {
                let fpu = answer(
                    format!("FPU type ({})", "FOR ALL BUT ARM-V8 ARCHITECTURES".bold()),
                    Some("".to_string()),
                )?;
                if fpu.is_empty() {
                    None
                } else {
                    Some(fpu)
                }
            },
        })
    }

    pub fn to_env_map(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        map.insert("LFA_ARM_ARCH", self.bits.to_string());
        map.insert("LFA_HOST", self.lfa_host.clone());
        map.insert("LFA_TGT", self.lfa_tgt.clone());
        map.insert("LFA_ARCH", self.lfa_arch.clone());

        if let Some(lfa_float) = self.lfa_float.clone() {
            map.insert("LFA_FLOAT", lfa_float);
        }

        if let Some(lfa_fpu) = self.lfa_fpu.clone() {
            map.insert("LFA_FPU", lfa_fpu);
        }

        map
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Bits {
    #[serde(rename = "arm")]
    Arm32,

    #[serde(rename = "arm64")]
    Arm64,
}

impl ToString for Bits {
    fn to_string(&self) -> String {
        match self {
            Self::Arm32 => "arm",
            Self::Arm64 => "arm64",
        }
        .to_string()
    }
}

impl FromStr for Bits {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "arm" | "arm32" | "32" => Ok(Self::Arm32),
            "arm64" | "64" => Ok(Self::Arm64),
            _ => Err(format!("arch type \"{}\" is incorrect!", s.bold().red())),
        }
    }
}

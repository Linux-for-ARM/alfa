//! Information about build process

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use toml;

// NOTE: можно использовать файл `packages.toml` из руководства LFA
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageList {
    pub package: HashMap<String, Package>,
}

impl PackageList {
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

// Информация о пакете
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    pub version: String,
    pub download: String,
    pub md5: String,
}

/// Порядок сборки пакетов
///
/// Указываются имена TOML-конфигов `PackageMeta` **без** расширения `*.toml`
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageOrder {
    pub packages: Vec<String>, // e.g. 'cross-compiler/linux-headers'

    /// В какой директории (полный путь) содержатся сборочные инструкции?
    pub prefix: String,
}

impl PackageOrder {
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

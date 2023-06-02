use eframe::emath::Numeric;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter};

pub mod file;
pub mod receiver;

#[allow(dead_code)]
#[derive(
    Debug, Clone, Copy, Display, PartialEq, PartialOrd, EnumCount, EnumIter, Serialize, Deserialize,
)]
#[repr(i32)]
pub enum LogLevel {
    None = 0x0,
    Fatal = 0x1,
    Error = 0x2,
    Warning = 0x4,
    Message = 0x8,
    Info = 0x10,
    Debug = 0x20,
    All = 0x3F,
}

impl Numeric for LogLevel {
    const INTEGRAL: bool = true;

    const MIN: Self = LogLevel::None;

    const MAX: Self = LogLevel::All;

    // this is needed for egui slider
    fn to_f64(self) -> f64 {
        match self {
            LogLevel::None => 0.0,
            LogLevel::Fatal => 1.0,
            LogLevel::Error => 2.0,
            LogLevel::Warning => 3.0,
            LogLevel::Message => 4.0,
            LogLevel::Info => 5.0,
            LogLevel::Debug => 6.0,
            LogLevel::All => 7.0,
        }
    }

    // this is needed for egui slider
    fn from_f64(num: f64) -> Self {
        match num {
            x if x >= 0.0 && x < 1.0 => LogLevel::None,
            x if x >= 1.0 && x < 2.0 => LogLevel::Fatal,
            x if x >= 2.0 && x < 3.0 => LogLevel::Error,
            x if x >= 3.0 && x < 4.0 => LogLevel::Warning,
            x if x >= 4.0 && x < 5.0 => LogLevel::Message,
            x if x >= 5.0 && x < 6.0 => LogLevel::Info,
            x if x >= 6.0 && x < 7.0 => LogLevel::Debug,
            x if x >= 7.0 && x < 8.0 => LogLevel::All,
            _ => LogLevel::All,
        }
    }
}

#[derive(Clone)]
pub struct BepInExLogEntry {
    level: LogLevel,
    data: String,
    data_lowercase: String,
    pub is_selected: bool,
}

impl BepInExLogEntry {
    pub fn new(level: LogLevel, data: String) -> Self {
        Self {
            level,
            data: data.clone(),
            data_lowercase: data.to_lowercase(),
            is_selected: false,
        }
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    pub fn data_lowercase(&self) -> &str {
        self.data_lowercase.as_ref()
    }
}

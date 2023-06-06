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

    const MIN: Self = Self::None;

    const MAX: Self = Self::All;

    // this is needed for egui slider
    fn to_f64(self) -> f64 {
        match self {
            Self::None => 0.0,
            Self::Fatal => 1.0,
            Self::Error => 2.0,
            Self::Warning => 3.0,
            Self::Message => 4.0,
            Self::Info => 5.0,
            Self::Debug => 6.0,
            Self::All => 7.0,
        }
    }

    // this is needed for egui slider
    fn from_f64(num: f64) -> Self {
        match num {
            x if (0.0..1.0).contains(&x) => Self::None,
            x if (1.0..2.0).contains(&x) => Self::Fatal,
            x if (2.0..3.0).contains(&x) => Self::Error,
            x if (3.0..4.0).contains(&x) => Self::Warning,
            x if (4.0..5.0).contains(&x) => Self::Message,
            x if (5.0..6.0).contains(&x) => Self::Info,
            x if (6.0..7.0).contains(&x) => Self::Debug,
            x if (7.0..8.0).contains(&x) => Self::All,
            _ => Self::All,
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
    pub fn new(level: LogLevel, data: &str) -> Self {
        Self {
            level,
            data: data.to_string(),
            data_lowercase: data.to_lowercase(),
            is_selected: false,
        }
    }

    pub const fn level(&self) -> LogLevel {
        self.level
    }

    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    pub fn data_lowercase(&self) -> &str {
        self.data_lowercase.as_ref()
    }
}

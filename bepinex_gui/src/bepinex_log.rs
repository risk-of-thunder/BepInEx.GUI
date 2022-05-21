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

pub struct BepInExLog {
    pub level: LogLevel,
    pub data: String,
}

impl BepInExLog {
    pub fn new(level: LogLevel, data: String) -> Self {
        Self { level, data }
    }
}

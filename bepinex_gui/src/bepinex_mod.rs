pub struct BepInExMod {
    pub name: String,
    pub version: String,
}

impl ToString for BepInExMod {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.version)
    }
}

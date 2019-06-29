#[derive(PartialOrd, PartialEq)]
pub enum Verbosity {
    Normal = 0,
    Info = 1,
    Debug = 2,
    Trace = 3,
}

impl Verbosity {
    pub fn from_int(level: u64) -> Self {
        match level {
            0 => Verbosity::Normal,
            1 => Verbosity::Info,
            2 => Verbosity::Debug,
            3 | _ => Verbosity::Trace,
        }
    }
}

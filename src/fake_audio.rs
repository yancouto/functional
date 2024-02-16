use enum_map::Enum;

#[derive(strum::Display, Debug, Enum, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
pub enum SFX {
    Back,
    Confirm,
    Select,
    Win,
    Wrong,
}

impl SFX {
    pub fn play(self) {}
}

pub fn tick() {}

pub fn set_volume(_vol: u8) {}

#[derive(Debug)]
pub enum PresetColor {
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Purple,
}

#[derive(Debug)]
pub enum Color {
    Preset(PresetColor),
    Color(String),
}

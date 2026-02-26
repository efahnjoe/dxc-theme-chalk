use dioxus::prelude::{Asset, asset, manganis};

pub const DXC_THEMES: Asset = asset!("../out/index.css");

pub const DXC_THEMES_STR: &str = include_str!("../out/index.css");

pub const DXC_THEMES_BYTES: &[u8] = DXC_THEMES_STR.as_bytes();

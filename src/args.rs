use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "TuFGamingM5 Driver")]
#[command(author = "Bijan Regmi")]
#[command(version = "0.1.0")]
#[command(about = "Manage settings for your TUF Gaming M5 mouse", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

use std::u32;

fn parse_hex_values(s: &str) -> Result<u32, String> {
    let err_msg = format!("`{s} isn't a valid hex color.`");

    if s.len() != 6 {
        return Err(err_msg);
    }

    u32::from_str_radix(s, 16).map_err(|_| err_msg)
}

#[derive(Subcommand)]
pub enum Commands {
    /// Change active profile
    Profile {
        /// Either 1 or 2 or 3
        #[arg(value_parser = clap::value_parser!(u8).range(1..4))]
        profile_number: u8,
    },

    Lighting {
        /// Lighting mode
        #[arg(short, long)]
        mode: LightingMode,

        /// Brightness level
        #[arg(short, long)]
        brightness: BrightnessLevel,

        /// Color in hex format (eg: a5ff01)
        #[arg(short, long, value_parser = parse_hex_values)]
        color: u32,
    },

    #[group(required = true, multiple = true)]
    Performance {
        /// Primary DPI value
        #[arg(long)]
        dpi1: Option<u16>,

        /// Secondary DPI value
        #[arg(long)]
        dpi2: Option<u16>,

        /// Polling rate (in hertz)
        #[arg(short, long)]
        polling: Option<u16>,

        /// Set button response time (in ms)
        #[arg(short, long)]
        response: Option<u8>,

        /// Enable or disable angle snapping
        #[arg(short, long)]
        snapping: Option<AngleSnapping>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AngleSnapping {
    Off,
    On,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LightingMode {
    Static = 0x00,
    Breathing = 0x01,
    ColorCycle = 0x02,
    Reactive = 0x03,
    AuraSync = 0x04,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BrightnessLevel {
    Off = 0x00,
    Low = 0x01,
    Medium = 0x02,
    High = 0x03,
    Full = 0x04,
}

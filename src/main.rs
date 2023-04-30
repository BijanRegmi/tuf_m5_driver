mod args;
mod mouse;

use args::{AngleSnapping, Args, Commands};
use clap::Parser;
use mouse::M5;

fn main() {
    let args = Args::parse();
    let mero_mouse = M5::new(0x0b05, 0x1898);

    match &args.command {
        Commands::Profile { profile_number } => mero_mouse.set_profile(*profile_number),

        Commands::Lighting {
            mode,
            brightness,
            color,
        } => mero_mouse.set_lighting(*mode as u8, *brightness as u8, *color),

        Commands::Performance {
            dpi1,
            dpi2,
            polling,
            response,
            snapping,
        } => {
            if let Some(d1) = dpi1 {
                mero_mouse.set_dpi1(*d1);
            }
            if let Some(d2) = dpi2 {
                mero_mouse.set_dpi2(*d2);
            }
            if let Some(p) = polling {
                mero_mouse.set_polling_rate(*p);
            }
            if let Some(r) = response {
                mero_mouse.set_button_response(*r);
            }
            if let Some(s) = snapping {
                mero_mouse.set_angle_snapping(if *s == AngleSnapping::On { true } else { false });
            }
        }
    }
}

use clap::builder::Styles;
use clap::builder::styling::{Color, RgbColor, Style};

const PRIMARY: Color = Color::Rgb(RgbColor(249, 65, 68));
const SECONDARY: Color = Color::Rgb(RgbColor(243, 114, 44));
const LIGHT_ORANGE: Color = Color::Rgb(RgbColor(248, 150, 30));
const _DARK_ORANGE: Color = Color::Rgb(RgbColor(249, 132, 74));
const YELLOW: Color = Color::Rgb(RgbColor(249, 199, 79));
const GREEN: Color = Color::Rgb(RgbColor(144, 190, 109));
const _LIGHT_CYAN: Color = Color::Rgb(RgbColor(67, 170, 139));
const CYAN: Color = Color::Rgb(RgbColor(77, 144, 142));
const _GRAY: Color = Color::Rgb(RgbColor(87, 117, 144));
const _BLUE: Color = Color::Rgb(RgbColor(39, 125, 161));

pub fn get_styles() -> Styles {
    Styles::styled()
        .invalid(
            Style::new().
                bold()
                .underline()
                .fg_color(Some(PRIMARY)))
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(SECONDARY))
        )
        .literal(
            Style::new()
                .fg_color(Some(LIGHT_ORANGE))
        )
        .valid(
            Style::new()
                .underline()
                .bold()
                .fg_color(Some(GREEN))
        )
        .placeholder(
            Style::new()
                .fg_color(Some(CYAN))
        )
        .header(
            Style::new()
                .bold()
                .fg_color(Some(YELLOW))
        )
}
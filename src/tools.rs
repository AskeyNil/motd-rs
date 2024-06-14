use byte_unit::{Byte, UnitType};
use termion::{color, style};

pub trait ByteStr {
    fn byte_str(&self) -> String;
}

impl ByteStr for u64 {
    fn byte_str(&self) -> String {
        format!(
            "{:.1}",
            Byte::from_u64(*self).get_appropriate_unit(UnitType::Binary)
        )
    }
}

pub fn color_to_rgb8(color: &str) -> Result<(u8, u8, u8), csscolorparser::ParseColorError> {
    let color = csscolorparser::parse(&color);
    let color = color?.to_rgba8();
    Ok((color[0], color[1], color[2]))
}

pub fn process_str(
    width: usize,
    per: f64,
    normal_color: &str,
    warning_color: &str,
    threshold: f64,
) -> String {
    let normal_color = color_to_rgb8(normal_color).expect(&format!(
        "normal_color = \"{}\" is not supported.",
        normal_color
    ));

    let warning_color = color_to_rgb8(warning_color).expect(&format!(
        "warning_color = \"{}\" is not supported.",
        warning_color
    ));

    let normal_color = color::Rgb(normal_color.0, normal_color.1, normal_color.2);
    let warning_color = color::Rgb(warning_color.0, warning_color.1, warning_color.2);
    let color;
    if per > threshold {
        color = color::Fg(warning_color);
    } else {
        color = color::Fg(normal_color);
    }
    let reset = style::Reset;

    let total = width - 2;
    let used = (total as f64 * per) as usize;
    let free = total - used;

    format!(
        "[{color}{}{reset}{}]",
        std::iter::repeat("=").take(used).collect::<String>(),
        std::iter::repeat("=").take(free).collect::<String>()
    )
}

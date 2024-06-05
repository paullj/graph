use rusttype::{Font, Scale};
use svg::Node;

pub(crate) trait ToSvg<T>
where
    T: Into<Box<dyn Node>>,
{
    fn to_svg(&mut self) -> T;
}

pub(crate) fn measure_text_width(content: &str, font_size: f32) -> f32 {
    // Load the font
    let font_data = include_bytes!("../fonts/SpaceMono-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(font_size);

    // Calculate the width of the text
    let width = font
        .glyphs_for(content.chars())
        .map(|g| g.scaled(scale).h_metrics().advance_width)
        .sum::<f32>();

    width
}

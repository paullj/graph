use resvg::usvg::{self, Options};
use svg::Node;

pub(crate) trait ToSvg<T>
where
    T: Into<Box<dyn Node>>,
{
    fn to_svg(&mut self) -> T;
}

use usvg::Tree;

pub(crate) fn measure_text_width(content: &str, font_size: f32) -> (f32, f32) {
    let mut opt = Options::default();
    let font_data = include_bytes!("../fonts/JetBrainsMono-Light.ttf");
    opt.fontdb_mut().load_system_fonts();
    opt.fontdb_mut().load_font_data(font_data.to_vec());
    opt.font_family = "JetBrains Mono".to_string();

    let tree = match Tree::from_str(
        format!(
            r###"
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 64"><g>
                <text font-size="{}px">{}</text></g>
            </svg>
            "###,
            font_size, content
        )
        .as_str(),
        &opt,
    ) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Failed to create SVG tree");
        }
    };

    let root = tree.root();
    (root.bounding_box().width(), root.bounding_box().height())
}

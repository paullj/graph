mod edge;
mod graph;
mod node;
mod parser;
mod render;

use base64::{engine::general_purpose, Engine as _};
use parser::parse_from_string;
use render::ToSvg;
use svg::{
    node::element::{Definitions, Marker, Polygon, Style},
    Document,
};

pub(crate) trait Position {
    fn set_position(&mut self, position: (isize, isize));
}

pub fn generate_graph(contents: &str) -> Result<String, String> {
    match parse_from_string(&contents) {
        Ok(mut graph) => {
            let mut graph_group = graph.to_svg();

            let (width, height) = graph.get_size();
            let margin = f32::max(20.0, f32::max(width, height) * 0.075);

            graph_group = graph_group.set("transform", format!("translate({margin}, {margin})"));

            let font_data = include_bytes!("../fonts/JetBrainsMono-Light.ttf");
            let font_data_base64 = general_purpose::STANDARD.encode(font_data);

            let right_arrow = Marker::new()
                .set("id", "right-arrow")
                .set("markerWidth", 5)
                .set("markerHeight", 5)
                .set("refX", 0)
                .set("refY", 2.5)
                .set("orient", "auto")
                .set("fill", "#5d5b5d")
                .add(Polygon::new().set("points", "0 0, 5 2.5, 0 5"));

            let left_arrow = Marker::new()
                .set("id", "left-arrow")
                .set("markerWidth", 5)
                .set("markerHeight", 5)
                .set("refX", 5)
                .set("refY", 2.5)
                .set("orient", "auto")
                .set("fill", "#5d5b5d")
                .add(Polygon::new().set("points", "5 0, 0 2.5, 5 5"));

            let document = Document::new()
                .set(
                    "viewBox",
                    (0, 0, width + margin * 2.0, height + margin * 2.0),
                )
                .add(
                    Definitions::new()
                        .add(left_arrow)
                        .add(right_arrow)
                        .add(Style::new(format!(
                    "@font-face {{
                        font-family: 'JetBrains Mono';
                        src: url(data:font/ttf;base64,{}) format('truetype');
                    }}
                    text {{
                        font-family: 'JetBrains Mono';
                    }}
                    svg {{
                        background-image: url(\"data:image/svg+xml,<svg id='patternId' width='100%' height='100%' xmlns='http://www.w3.org/2000/svg'><defs><pattern id='a' patternUnits='userSpaceOnUse' width='100' height='100' patternTransform='scale(0.3) rotate(0)'><rect x='0' y='0' width='100%' height='100%' fill='hsla(335,50%,99%,1)'/><path d='M11 6a5 5 0 01-5 5 5 5 0 01-5-5 5 5 0 015-5 5 5 0 015 5'  stroke-width='1' stroke='none' fill='hsla(317, 20%, 90%, 1)'/></pattern></defs><rect width='800%' height='800%' transform='translate(0,0)' fill='url(%23a)'/></svg>\")
                    }}
                    ",
                    font_data_base64
                ))))
                .add(graph_group);

            // background-image:
            Ok(document.to_string())
        }
        Err(e) => {
            eprintln!("{:?}", e);
            Err(e.to_string())
        }
    }
}

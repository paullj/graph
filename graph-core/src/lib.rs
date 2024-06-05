mod edge;
mod graph;
mod node;
mod parser;
mod render;

use base64::{engine::general_purpose, Engine as _};
use parser::parse_from_string;
use render::ToSvg;
use svg::{
    node::element::{Definitions, Style},
    Document,
};

pub(crate) trait Position {
    fn set_position(&mut self, position: (isize, isize));
}

pub fn generate_graph(contents: &str) -> Result<String, String> {
    match parse_from_string(&contents) {
        Ok(mut graph) => {
            let margin = 20.0;
            let graph_group = graph
                .to_svg()
                .set("transform", format!("translate({margin}, {margin})"));

            let (width, height) = graph.get_size();

            let font_data = include_bytes!("../fonts/SpaceMono-Regular.ttf");
            let font_data_base64 = general_purpose::STANDARD.encode(font_data);

            let document = Document::new()
                .set(
                    "viewBox",
                    (0, 0, width + margin * 2.0, height + margin * 2.0),
                )
                .add(Definitions::new().add(Style::new(format!(
                    "@font-face {{
                        font-family: 'Space Mono';
                        src: url(data:font/ttf;base64,{}) format('truetype');
                    }}
                    text {{
                        font-family: 'Space Mono';
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

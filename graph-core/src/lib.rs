mod edge;
mod graph;
mod node;
mod parser;
mod render;

use parser::parse_from_string;
use render::ToSvg;
use svg::Document;

pub(crate) trait Position {
    fn set_position(&mut self, position: (isize, isize));
}

pub fn generate_graph(contents: &str) -> Result<String, String> {
    match parse_from_string(&contents) {
        Ok(graph) => {
            let document = Document::new()
                .set("viewBox", (0, 0, 1000, 1000))
                .add(graph.to_svg());
            Ok(document.to_string())
        }
        Err(e) => {
            eprintln!("{:?}", e);
            Err(e.to_string())
        }
    }
}

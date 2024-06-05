use resvg::usvg::{Node::Text as UText, Options, Tree};
use svg::{node::element::Text as TextElement, Document, Node};

pub(crate) trait ToSvg<T>
where
    T: Into<Box<dyn Node>>,
{
    fn to_svg(&self) -> T;
}

pub(crate) fn measure_text(content: &str, font_size: f32) -> (f32, f32) {
    let text = TextElement::new(content)
        .set("id", "text")
        .set("x", 0)
        .set("y", 0)
        .set("font-family", "monospace")
        .set("font-size", font_size);

    let document = Document::new()
        .set("width", 100)
        .set("height", 100)
        .add(text);
    let svg_string = document.to_string();
    println!("{}", svg_string);
    let opt = Options::default();
    let tree = Tree::from_str(&svg_string, &opt).unwrap();

    println!("{:?}", tree);
    if let Some(node) = tree.root().children().first() {
        println!("{:?}", node);
        if let UText(text) = &*node {
            let bbox = text.bounding_box();
            return (bbox.width(), bbox.height());
        }
    }

    (0.0, 0.0)
}

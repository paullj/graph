use crate::{
    parser::Rule,
    render::{measure_text, ToSvg},
    Position,
};
use pest::iterators::Pair;
use svg::node::element::{Element, Group, Rectangle, Text};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Node {
    pub id: String,
    pub label: Option<String>,
    pub shape: NodeShape,
    pub position: Option<(isize, isize)>,
}

impl Position for Node {
    fn set_position(&mut self, position: (isize, isize)) {
        self.position = Some(position);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NodeShape {
    Rounded,
    Square,
    Triangle,
    Empty,
}

impl From<&str> for NodeShape {
    fn from(shape: &str) -> Self {
        match shape {
            "(" => NodeShape::Rounded,
            "[" => NodeShape::Square,
            "{" => NodeShape::Triangle,
            _ => NodeShape::Empty,
        }
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: String::new(),
            label: None,
            shape: NodeShape::Empty,
            position: None,
        }
    }
}

impl ToSvg<Rectangle> for NodeShape {
    fn to_svg(&self) -> Rectangle {
        let base = Rectangle::new()
            .set("fill", "#D9D9D9")
            .set("stroke", "black")
            .set("stroke-width", 2);

        match self {
            NodeShape::Rounded => base.set("rx", 20),
            NodeShape::Square => base.set("rx", 10),
            NodeShape::Triangle => base,
            NodeShape::Empty => Rectangle::new(),
        }
    }
}

impl ToSvg<Group> for Node {
    fn to_svg(&self) -> Group {
        const MARGIN: f32 = 10.0;

        let id_string = self.id.clone();

        // let (id_width, id_height) = measure_text(id_string.as_str(), 12.0);

        let (x, y) = match self.position {
            Some((x, y)) => (x, y),
            None => (0, 0),
        };

        let id = Text::new(id_string)
            .set("font-family", "monospace")
            .set("font-size", 10)
            .set("text-anchor", "left");

        let shape = self
            .shape
            .to_svg()
            .set("width", 100.0 + MARGIN * 2.0)
            .set("height", 20.0 + MARGIN * 2.0);

        let mut group = Group::new().set("id", self.id.clone()).add(shape).add(id);
        if let Some((x, y)) = self.position {
            group = group.set("transform", format!("translate({},{})", x, -y));
        }
        if let Some(label) = self.label.clone() {
            let label_text = Text::new(label)
                .set("font-family", "monospace")
                .set("font-size", 12)
                .set("x", MARGIN * 2.0)
                .set("y", MARGIN * 1.5)
                .set("text-anchor", "left");
            group = group.add(label_text);
        }

        // if let Some(label) = self.label.clone() {
        //     group.add(
        //         Text::new(label)
        //             .set("x", 50)
        //             .set("y", 50)
        //             .set("text-anchor", "middle"),
        //     );
        // }
        group
    }
}

impl<'a> From<Pair<'a, Rule>> for Node {
    fn from(pair: Pair<Rule>) -> Node {
        let mut node = Node::new();

        let node_pairs = pair.into_inner();
        for node_pair in node_pairs {
            let rule = node_pair.as_rule();
            match rule {
                Rule::id => {
                    node.id = node_pair.as_str().to_string();
                }
                Rule::node_label => {
                    let node_label = node_pair.as_str();
                    node.label = if node_label.is_empty() {
                        None
                    } else {
                        Some(String::from(node_label))
                    }
                }
                Rule::node_shape => {
                    node.shape = NodeShape::from(node_pair.as_str());
                }
                _ => {}
            }
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use rstest::rstest;

    fn get_node(input: &str) -> Pair<Rule> {
        use crate::parser::GraphParser;

        let parse_result = GraphParser::parse(Rule::node, &input);
        let pair = parse_result.unwrap().next().unwrap();

        pair
    }

    #[rstest]
    #[case("(", NodeShape::Rounded)]
    #[case("[", NodeShape::Square)]
    #[case("{", NodeShape::Triangle)]
    #[case(")", NodeShape::Empty)]
    #[case("x", NodeShape::Empty)]
    #[case("not a shape", NodeShape::Empty)]
    fn test_nodeshape_valid(#[case] input: &str, #[case] expected: NodeShape) {
        assert_eq!(NodeShape::from(input), expected);
    }

    #[rstest]
    #[case("a", "a", None, NodeShape::Empty)]
    #[case("a(A)", "a", Some("A"), NodeShape::Rounded)]
    #[case("a (a)", "a", None, NodeShape::Empty)]
    #[case("a[a]", "a", Some("a"), NodeShape::Square)]
    #[case("a{}", "a", None, NodeShape::Triangle)]
    fn test_node_from_pair(
        #[case] input: &str,
        #[case] id: &str,
        #[case] label: Option<&str>,
        #[case] shape: NodeShape,
    ) {
        let pair = get_node(input);
        let node = Node::from(pair);
        assert_eq!(node.id, id);
        assert_eq!(node.label, label.map(String::from));
        assert_eq!(node.shape, shape);
    }
}

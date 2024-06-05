use crate::{
    parser::Rule,
    render::{measure_text_width, ToSvg},
};
use pest::iterators::Pair;
use svg::node::element::{Group, Rectangle, Text};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Node {
    pub id: String,
    pub label: Option<String>,
    pub shape: NodeShape,
    pub position: Option<(isize, isize)>,
    pub size: Option<(f32, f32)>,
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
            size: None,
        }
    }

    pub fn max_text_width(&self) -> f32 {
        let mut max_text_width = measure_text_width(&self.id, 8.0);

        if let Some(label) = &self.label {
            let label_width = measure_text_width(&label, 12.0);
            max_text_width = f32::max(max_text_width, label_width);
        }
        max_text_width
    }
}

impl ToSvg<Rectangle> for NodeShape {
    fn to_svg(&mut self) -> Rectangle {
        let base = Rectangle::new()
            .set("fill", "#FBFCFC")
            .set("stroke", "#E2E6EA")
            .set("stroke-width", 1);

        match self {
            NodeShape::Rounded => base.set("rx", 10),
            NodeShape::Square => base.set("rx", 2),
            NodeShape::Triangle => base,
            NodeShape::Empty => Rectangle::new(),
        }
    }
}

impl ToSvg<Group> for Node {
    fn to_svg(&mut self) -> Group {
        let id_string = self.id.clone();

        let padding_x = 10.0;
        let (x, y) = match self.position {
            Some(position) => position,
            None => (0, 0),
        };

        let max_text_width = self.max_text_width();

        self.size = Some((max_text_width as f32 + padding_x * 2.0, 50.0));

        let id = Text::new(id_string)
            .set("font-size", "8px")
            .set("text-anchor", "left")
            .set("x", padding_x)
            .set("y", 8);

        let shape = self
            .shape
            .to_svg()
            .set("width", max_text_width)
            .set("height", 50);

        let mut group = Group::new().set("id", self.id.clone()).add(shape).add(id);
        group = group.set("transform", format!("translate({},{})", x, y));

        if let Some(label) = self.label.clone() {
            let label_text = Text::new(label)
                .set("font-size", "12px")
                .set("x", padding_x)
                .set("y", 20)
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

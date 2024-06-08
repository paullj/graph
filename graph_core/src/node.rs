use crate::{
    parser::Rule,
    render::{measure_text_width, ToSvg},
};
use pest::iterators::Pair;
use svg::node::element::{ClipPath, Definitions, Group, Line, Rectangle, Text};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Node {
    pub id: String,
    pub label: Option<String>,
    pub shape: NodeShape,
    pub position: Option<(f32, f32)>,
    pub size: Option<(f32, f32)>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NodeShape {
    Rounded,
    Square,
    Triangle,
    Empty,
}

impl Default for NodeShape {
    fn default() -> Self {
        NodeShape::Empty
    }
}

impl NodeShape {
    pub fn to_svg(&self, id: &str, width: f32, height: f32, stroke: f32, header: f32) -> Group {
        let mut group = Group::new().set("class", "node");
        match self {
            NodeShape::Rounded | NodeShape::Square => {
                let rx = match self {
                    NodeShape::Rounded => 12.0,
                    NodeShape::Square | _ => 4.0,
                };

                let clip = ClipPath::new().set("id", format!("clip_path_{}", id)).add(
                    Rectangle::new()
                        .set("width", width - stroke)
                        .set("height", height - stroke)
                        .set("x", stroke / 2.0)
                        .set("y", stroke / 2.0)
                        .set("rx", rx - stroke / 2.0),
                );

                let defs = Definitions::new().add(clip);
                group = group
                    .add(defs)
                    .add(
                        Rectangle::new()
                            .set("width", width)
                            .set("height", height)
                            .set("stroke-width", stroke)
                            .set("rx", rx)
                            .set("fill", "#fcf9fa")
                            .set("stroke", "#cecace"),
                    )
                    .add(
                        Rectangle::new()
                            .set("width", width - stroke)
                            .set("height", height - stroke)
                            .set("x", stroke / 2.0)
                            .set("y", stroke / 2.0)
                            .set("rx", rx - stroke / 2.0)
                            .set("fill", "#fcf9fa")
                            .set(
                                "clip-path",
                                format!("url(#{})", format!("clip_path_{}", id)),
                            ),
                    )
                    .add(
                        Line::new()
                            .set("x1", 0)
                            .set("y1", header)
                            .set("x2", width)
                            .set("y2", header)
                            .set("stroke", "#cecace")
                            .set("stroke-width", 1),
                    );
            }
            NodeShape::Triangle => {}
            NodeShape::Empty => {}
        }

        group
    }
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
        let (mut max_text_width, _id_text_height) = measure_text_width(&self.id, 8.0);

        if let Some(label) = &self.label {
            let (label_text_width, _label_text_height) = measure_text_width(&label, 12.0);
            max_text_width = f32::max(max_text_width, label_text_width);
        }
        max_text_width
    }
}

impl ToSvg<Group> for Node {
    fn to_svg(&mut self) -> Group {
        // Calculate sizes and positions for elements
        let padding = (10.0, 5.0);
        let (x, y) = match self.position {
            Some(position) => position,
            None => (0.0, 0.0),
        };
        let id_font_size = 6.0;
        let label_font_size = 8.0;
        let (id_text_width, id_text_height) = measure_text_width(&self.id, id_font_size);
        let (label_text_width, label_text_height) = match &self.label {
            Some(label) => measure_text_width(&label, label_font_size),
            None => (0.0, 0.0),
        };

        let size = (
            f32::max(id_text_width, label_text_width) + padding.0 * 2.0,
            label_text_height + id_text_height + padding.1 * 2.0,
        );

        let mut group = Group::new().set("id", self.id.clone()).set(
            "transform",
            format!("translate({},{})", x - size.0 / 2.0, y - size.1 / 2.0),
        );

        let id = Text::new(&self.id)
            .set("font-size", format!("{}px", id_font_size))
            .set("x", padding.0)
            .set("y", id_text_height);

        let shape = self
            .shape
            .to_svg(&self.id, size.0, size.1, 1.0, id_text_height + padding.1);

        group = group.add(shape).add(id).set(
            "transform",
            format!("translate({}, {})", x - size.0 / 2.0, y - size.1 / 2.0),
        );

        if let Some(label) = &self.label {
            let label_text = Text::new(label)
                .set("font-size", format!("{}px", label_font_size))
                .set("x", padding.0)
                .set("y", id_text_height + label_text_height + padding.1);

            group = group.add(label_text);
        }

        self.size = Some(size);

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

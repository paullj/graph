use pest::iterators::Pair;

use crate::parser::Rule;

#[derive(Debug, PartialEq)]
pub(crate) struct Edge {
    pub label: Option<String>,
    pub line: EdgeLine,
    pub source_head: EdgeHead,
    pub target_head: EdgeHead,
}

#[derive(Debug, PartialEq)]
pub(crate) enum EdgeLine {
    Thin,
    Dotted,
    Thick,
    Wavy,
}

#[derive(Debug, PartialEq)]
pub(crate) enum EdgeHead {
    Left,
    Right,
    Straight,
    Dot,
    None,
}

impl From<&str> for EdgeHead {
    fn from(head: &str) -> Self {
        match head {
            "<" => EdgeHead::Left,
            ">" => EdgeHead::Right,
            "|" => EdgeHead::Straight,
            ":" => EdgeHead::Dot,
            _ => EdgeHead::None,
        }
    }
}

impl From<&str> for EdgeLine {
    fn from(line: &str) -> Self {
        match line {
            "-." => EdgeLine::Dotted,
            "~~" => EdgeLine::Wavy,
            "==" => EdgeLine::Thick,
            "--" | _ => EdgeLine::Thin,
        }
    }
}

impl Edge {
    fn new() -> Self {
        Self {
            label: None,
            line: EdgeLine::Thin,
            source_head: EdgeHead::None,
            target_head: EdgeHead::None,
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for Edge {
    fn from(pair: Pair<Rule>) -> Self {
        let mut edge = Edge::new();
        let edge_pairs = pair.into_inner();
        for edge_pair in edge_pairs {
            let rule = edge_pair.as_rule();
            match rule {
                Rule::edge_label => {
                    edge.label = Some(edge_pair.as_str().trim_matches('|').to_string())
                }
                Rule::source_head => edge.source_head = EdgeHead::from(edge_pair.as_str()),
                Rule::target_head => edge.target_head = EdgeHead::from(edge_pair.as_str()),
                Rule::line => edge.line = EdgeLine::from(edge_pair.as_str()),
                _ => {}
            }
        }
        edge
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use rstest::*;

    fn get_pair(input: &str) -> Pair<Rule> {
        use crate::parser::GraphParser;

        let parse_result = GraphParser::parse(Rule::edge, &input);
        let pair = parse_result.unwrap().next().unwrap();

        pair
    }

    #[rstest]
    #[case("a --> b")]
    #[case("a     --> b")]
    #[case("a -->     b")]
    #[case("a   -->   b")]
    fn test_edge_from_pair_whitespaces(#[case] input: &str) {
        let pair = get_pair(input);

        let edge = Edge::from(pair);
        assert_eq!(edge.label, None);
        assert_eq!(edge.line, EdgeLine::Thin);
        assert_eq!(edge.source_head, EdgeHead::None);
        assert_eq!(edge.target_head, EdgeHead::Right);
    }

    #[rstest]
    #[case("a --> b", None, EdgeLine::Thin, EdgeHead::None, EdgeHead::Right)]
    #[case("a <--> b", None, EdgeLine::Thin, EdgeHead::Left, EdgeHead::Right)]
    #[case("a == b", None, EdgeLine::Thick, EdgeHead::None, EdgeHead::None)]
    #[case("a ~~ b", None, EdgeLine::Wavy, EdgeHead::None, EdgeHead::None)]
    #[case("a <~~ b", None, EdgeLine::Wavy, EdgeHead::Left, EdgeHead::None)]
    #[case("a <-. b", None, EdgeLine::Dotted, EdgeHead::Left, EdgeHead::None)]
    #[case("a :--: b", None, EdgeLine::Thin, EdgeHead::Dot, EdgeHead::Dot)]
    #[case("a |--> b", None, EdgeLine::Thin, EdgeHead::Straight, EdgeHead::Right)]
    #[case(
        "a <-- |Edge Label| b",
        Some("Edge Label"),
        EdgeLine::Thin,
        EdgeHead::Left,
        EdgeHead::None
    )]
    #[case(
        "a --> |Edge Label| b",
        Some("Edge Label"),
        EdgeLine::Thin,
        EdgeHead::None,
        EdgeHead::Right
    )]
    fn test_edge_from_pair(
        #[case] input: &str,
        #[case] label: Option<&str>,
        #[case] line: EdgeLine,
        #[case] source_head: EdgeHead,
        #[case] target_head: EdgeHead,
    ) {
        let pair = get_pair(input);

        let edge = Edge::from(pair);
        assert_eq!(edge.label, label.map(String::from));
        assert_eq!(edge.line, line);
        assert_eq!(edge.source_head, source_head);
        assert_eq!(edge.target_head, target_head);
    }
}

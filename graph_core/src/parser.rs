use crate::edge::Edge;
use crate::graph::{Graph, GraphBuilder};
use crate::node::Node;
use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "graph.pest"]
pub struct GraphParser;

pub fn parse_from_string(contents: &str) -> Result<Graph, Error<Rule>> {
    let pairs = match GraphParser::parse(Rule::graph, &contents) {
        Ok(mut parse_result) => parse_result.next().unwrap().into_inner(),

        Err(e) => {
            return Err(e);
        }
    };

    let mut builder = GraphBuilder::new();

    for graph_pair in pairs {
        match graph_pair.as_rule() {
            Rule::definition => {
                for definition_pair in graph_pair.into_inner() {
                    match definition_pair.as_rule() {
                        Rule::direction => {
                            // println!("Direction: {}", definition_pair.as_str())
                        }
                        _ => {}
                    }
                }
            }
            Rule::statement => {
                for statement_pair in graph_pair.into_inner() {
                    match statement_pair.as_rule() {
                        Rule::node => {
                            builder.insert_or_update_node(Node::from(statement_pair));
                        }
                        Rule::edge => {
                            let mut source_id = None;
                            let mut target_id = None;
                            for pair in statement_pair.clone().into_inner() {
                                match pair.as_rule() {
                                    Rule::node => {
                                        if let Some(tag) = pair.as_node_tag() {
                                            let node = Node::from(pair.clone());
                                            let node_id = node.id.clone();
                                            match tag {
                                                "source" => source_id = Some(node_id),
                                                "target" => target_id = Some(node_id),
                                                _ => {}
                                            }
                                            builder.insert_node(node);
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            match (source_id, target_id) {
                                (Some(source), Some(target)) => {
                                    let edge = Edge::from(statement_pair);
                                    builder.add_edge(source, target, edge);
                                }
                                _ => panic!("Edge must have source and target nodes"),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Rule::EOI => break,
            _ => {}
        }
    }

    let graph = builder.build();

    Ok(graph)
}

use std::collections::HashMap;

use petgraph::{graph::NodeIndex, stable_graph::StableGraph};
use rust_sugiyama::{self, CrossingMinimization, RankingType};
use svg::node::element::Group;

use crate::{
    edge::{self, Edge},
    node::Node,
    render::ToSvg,
};

pub(crate) struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }

    pub fn get_size(&self) -> (f32, f32) {
        let (mut width, mut height) = (100.0, 100.0);

        for node in self.nodes.iter() {
            match (node.position, node.size) {
                (Some(position), Some(size)) => {
                    let (x, y) = position;
                    let (w, h) = size;
                    width = f32::max(x as f32 + w, width);
                    height = f32::max(y as f32 + h, height);
                }
                _ => (),
            }
        }

        (width, height)
    }
}

impl ToSvg<Group> for Graph {
    fn to_svg(&mut self) -> svg::node::element::Group {
        let mut group = svg::node::element::Group::new().set("id", "graph");
        for node in self.nodes.iter_mut() {
            group = group.add(node.to_svg());
        }
        for edge in self.edges.iter_mut() {
            group = group.add(edge.to_svg());
        }
        group.into()
    }
}

pub(crate) struct GraphBuilder {
    node_map: HashMap<String, Node>,
    edge_map: HashMap<(String, String), Edge>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            node_map: HashMap::new(),
            edge_map: HashMap::new(),
        }
    }

    pub fn build(&mut self) -> Graph {
        let mut raw_graph = StableGraph::new();
        let mut node_indexes: HashMap<String, NodeIndex> = HashMap::new();

        // TODO: Set up minimum node spacing
        let mut max_width = 50.0;
        let spacing = 1.5;

        for node in self.node_map.values() {
            let node_id = node.id.clone();
            node_indexes.insert(node.id.clone(), raw_graph.add_node(node_id));
            max_width = f32::max(max_width, node.max_text_width());
        }
        for ((source_id, target_id), edge) in self.edge_map.iter() {
            let source_index = node_indexes.get(source_id);
            let target_index = node_indexes.get(target_id);

            match (source_index, target_index) {
                (Some(source), Some(target)) => {
                    raw_graph.add_edge(*source, *target, edge);
                }
                _ => println!("Edge must have source and target nodes"),
            }
        }

        let layouts = rust_sugiyama::from_graph(&raw_graph)
            .vertex_spacing((max_width * spacing) as usize)
            .minimum_length(1)
            .crossing_minimization(CrossingMinimization::Barycenter)
            .layering_type(RankingType::MinimizeEdgeLength)
            .build()
            .into_iter()
            .map(|(layout, width, height)| {
                let mut new_layout = HashMap::new();
                for (id, coords) in layout {
                    if let Some(node) = raw_graph.node_weight(NodeIndex::from(id)) {
                        new_layout.insert(node, coords);
                    }
                }
                (new_layout, width, height)
            })
            .collect::<Vec<_>>();

        for (positions, _, _) in layouts {
            for (node_id, position) in positions {
                if let Some(node) = self.node_map.get_mut(node_id) {
                    node.position = Some((position.0 as f32, -position.1 as f32));
                    node.calculate_size();
                }
            }
        }
        for ((source_id, target_id), edge) in self.edge_map.iter_mut() {
            if let Some(source) = self.node_map.get(source_id) {
                if let Some(target) = self.node_map.get(target_id) {
                    let (sx, sy) = source.position.unwrap();
                    let (sw, sh) = source.size.unwrap();
                    let (tx, ty) = target.position.unwrap();
                    let (tw, th) = target.size.unwrap();

                    let source_head_offset = match edge.source_head {
                        edge::EdgeHead::Left | edge::EdgeHead::Right => 7.5,
                        _ => 3.0,
                    };
                    let target_head_offset = match edge.target_head {
                        edge::EdgeHead::Left | edge::EdgeHead::Right => 7.5,
                        _ => 3.0,
                    };

                    // Calculate the direction of the edge
                    let dx = tx - sx - f32::abs(source_head_offset - target_head_offset);
                    let dy = ty - sy - f32::abs(source_head_offset - target_head_offset);
                    let length = (dx * dx + dy * dy).sqrt();

                    // Normalize the direction
                    let dx = dx / length;
                    let dy = dy / length;

                    // Calculate the new start and end positions
                    let start_x = sx + sw / 2.0 + dx * (sw / 2.0 + source_head_offset);
                    let start_y = sy + sh / 2.0 + (dy * sh / 2.0 + source_head_offset);
                    let end_x = tx + tw / 2.0 - (dx * tw / 2.0 + target_head_offset);
                    let end_y = ty + th / 2.0 - (dy * th / 2.0 + target_head_offset);

                    edge.position = Some((start_x, start_y, end_x, end_y));
                }
            }
        }

        Graph::new(
            self.node_map.values().cloned().collect(),
            self.edge_map.values().cloned().collect(),
        )
    }

    pub fn insert_node(&mut self, node: Node) -> &mut Self {
        if !self.node_map.contains_key(&node.id) {
            self.node_map.insert(node.id.clone(), node);
        }
        self
    }

    pub fn insert_or_update_node(&mut self, node: Node) -> &mut Self {
        self.node_map.insert(node.id.clone(), node);
        self
    }

    pub fn add_edge(&mut self, source: String, target: String, edge: Edge) -> &mut Self {
        self.edge_map.insert((source, target), edge);
        self
    }
}

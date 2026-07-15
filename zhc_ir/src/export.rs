use std::path::Path;
use serde::Serialize;
use zhc_utils::FastMap;

use crate::{Dialect, DisplayFormat, IR};

#[derive(Debug, Clone, Serialize)]
pub struct NodeLinkGraph {
    pub directed: bool,
    pub multigraph: bool,
    pub graph: GraphMeta,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphMeta {
    pub dialect: String,
    pub n_ops: u32,
    pub n_vals: u32,
    pub depth: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub id: u32,
    pub op: String,
    pub sig: String,
    pub depth: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub source: u32,
    pub target: u32,
    pub key: u32,
    pub valid: u32,
    pub out_pos: u16,
    pub in_pos: u16,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl<D: Dialect> IR<D> {
    pub fn to_node_link(&self) -> NodeLinkGraph {
        let nodes = self
            .walk_ops_topological()
            .map(|op| {
                let args = op
                    .get_args_iter()
                    .map(|v| v.get_type().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let rets = op
                    .get_returns_iter()
                    .map(|v| v.get_type().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                Node {
                    id: op.get_id().0,
                    op: format!("{}", DisplayFormat(&op.get_instruction())),
                    sig: format!("({args}) -> ({rets})"),
                    depth: op.get_depth(),
                    comment: op.get_comment().map(str::to_owned),
                }
            })
            .collect();

        let mut keys: FastMap<(u32, u32), u32> = FastMap::new();
        let mut links = Vec::new();
        for val in self.walk_vals_linear() {
            let origin = val.get_origin();
            let source = origin.opref.get_id().0;
            let valid = val.get_id().0;
            let typ = val.get_type().to_string();
            for use_ in val.get_uses_iter() {
                let target = use_.opref.get_id().0;
                let key = keys.entry((source, target)).or_insert(0);
                links.push(Link {
                    source,
                    target,
                    key: *key,
                    valid,
                    out_pos: origin.position,
                    in_pos: use_.position,
                    r#type: typ.clone(),
                });
                *key += 1;
            }
        }

        NodeLinkGraph {
            directed: true,
            multigraph: true,
            graph: GraphMeta {
                dialect: std::any::type_name::<D>().to_owned(),
                n_ops: self.n_ops(),
                n_vals: self.n_vals(),
                depth: self.depth(),
            },
            nodes,
            links,
        }
    }

    pub fn to_node_link_json(&self) -> String {
        serde_json::to_string_pretty(&self.to_node_link())
            .expect("node-link serialization is infallible")
    }

    pub fn write_node_link_json(&self, path: impl AsRef<Path>) {
        std::fs::write(path, self.to_node_link_json()).expect("Failed to write node-link JSON file");
    }
}

use crate::app::Node;
use scylla::cluster::Node as ScyllaNode;
use std::sync::Arc;

pub(crate) fn transform_nodes(current_nodes: &[Arc<ScyllaNode>]) -> Vec<Node> {
    current_nodes
        .iter()
        .map(move |node| {
            let datacenter = node.datacenter.clone().unwrap();
            let name = node.host_id.clone().to_string();
            let address = node.address.to_string();
            let coords = match datacenter.as_str() {
                "SA-DC" => (-23.5505, -46.6333),
                "EU-DC" => (52.5200, 13.4050),
                _ => (0.0, 0.0),
            };
            let status = match !node.is_down() {
                true => "Up",
                false => "Down",
            };

            Node {
                name,
                datacenter,
                coords,
                address,
                status: status.to_string(),
            }
        })
        .collect()
}

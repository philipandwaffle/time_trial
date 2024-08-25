mod blueprints;
mod bundles;
mod goal;
mod input;
mod level;
pub mod level_pack;
mod logic_graph;
pub mod manager;
mod output;
mod time_shift;

#[cfg(test)]
mod tests {
    use super::logic_graph::{LogicGraph, Node, Operator};

    #[test]
    fn or_logic_graph() {
        let nodes = vec![
            Node::new(Operator::None, vec![]),
            Node::new(Operator::None, vec![]),
            Node::new(Operator::Or, vec![0, 1]),
        ];

        let mut logic_graph = LogicGraph::new(vec![0, 1], vec![2], nodes);
        let out = logic_graph.process(vec![true, true]);
        assert!(out[0] == true);

        let out = logic_graph.process(vec![true, false]);
        assert!(out[0] == true);

        let out = logic_graph.process(vec![false, true]);
        assert!(out[0] == true);

        let out = logic_graph.process(vec![false, false]);
        assert!(out[0] == false);
    }

    #[test]
    fn and_logic_graph() {
        let nodes = vec![
            Node::new(Operator::None, vec![]),
            Node::new(Operator::None, vec![]),
            Node::new(Operator::And, vec![0, 1]),
        ];

        let mut logic_graph = LogicGraph::new(vec![0, 1], vec![2], nodes);
        let out = logic_graph.process(vec![true, true]);
        assert!(out[0] == true);

        let out = logic_graph.process(vec![true, false]);
        assert!(out[0] == false);

        let out = logic_graph.process(vec![false, true]);
        assert!(out[0] == false);

        let out = logic_graph.process(vec![false, false]);
        assert!(out[0] == false);
    }
}

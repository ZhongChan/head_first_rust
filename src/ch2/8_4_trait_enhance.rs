fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![("关联类型", Box::new(|| associated_types()))];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

/// # 关联类型
/// * 关联类型在 trait 中定义。
/// * trait 可以成为泛型的一部分
///
/// ## 基本用法
/// * trait 在引用一些类型时，这些类型与 trait 的实现具体相关；
/// * 关联类型通常用于定义共享行为的 trait 中。
fn associated_types() {
    let mut graph = AdjacencyMatrix {
            nodes: Vec::new(),
            edges: Vec::new(),
        };
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(10, &1, &2);
        graph.add_edge(20, &2, &3);

        println!("Node count: {}",graph.node_count());
        println!("Edge count: {}",graph.edge_count());
}

trait Graph {
    // 关联类型
    type Node;
    type Edge;

    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, edge: Self::Edge, from: &Self::Node, to: &Self::Node);
    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
}

struct AdjacencyMatrix {
    nodes: Vec<i32>,
    edges: Vec<Vec<Option<i32>>>,
}

impl Graph for AdjacencyMatrix {
    type Node = i32;
    type Edge = i32;

    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
        for ele in self.edges.iter_mut() {
            ele.push(None);
        }
        self.edges.push(vec![None; self.nodes.len()]);
    }

    fn add_edge(&mut self, edge: Self::Edge, from: &Self::Node, to: &Self::Node) {
        let from_index = self
            .nodes
            .iter()
            .position(|&n| n == *from)
            .expect("From node not found");
        let to_index = self
            .nodes
            .iter()
            .position(|&n| n == *to)
            .expect("To node not found");
        self.edges[from_index][to_index] = Some(edge);
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.iter().flat_map(|row| row.iter()).filter(|&e| e.is_some()).count()
    }
}

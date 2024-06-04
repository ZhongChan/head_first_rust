use std::ops::Add;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("关联类型", Box::new(|| associated_types())),
        ("默认泛型类型参数", Box::new(|| default_generic_params())),
    ];

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
    // 关联类型 实现邻接矩阵
    let mut graph = AdjacencyMatrix {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    graph.add_edge(10, &1, &2);
    graph.add_edge(20, &2, &3);

    println!("Node count: {}", graph.node_count());
    println!("Edge count: {}", graph.edge_count());

    // 泛型 实现邻接矩阵
    let mut graph_g = AdjacencyMatrixG::<i32, f32> {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let node1 = graph_g.add_node(1);
    let node2 = graph_g.add_node(2);
    let node3 = graph_g.add_node(3);
    graph_g.add_edge(10.0, node1, node2);
    graph_g.add_edge(20.1, node2, node3);

    println!("Node count: {}", graph_g.node_count());
    println!("Edge count: {}", graph_g.edge_count());

    // 关联类型 迭代器
    let mut counter = Counter::new(10);
    while let Some(x) = counter.next() {
        println!("{}", x);
    }
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
        self.edges
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&e| e.is_some())
            .count()
    }
}

trait GraphG<N, E> {
    fn add_node(&mut self, node: N) -> usize;
    fn add_edge(&mut self, edge: E, from: usize, to: usize);
    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
}

struct AdjacencyMatrixG<N, E> {
    nodes: Vec<N>,
    edges: Vec<Vec<Option<E>>>,
}

impl<N, E> GraphG<N, E> for AdjacencyMatrixG<N, E> {
    fn add_node(&mut self, node: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        for ele in self.edges.iter_mut() {
            ele.push(None);
        }

        //初始化空行
        let mut new_row = Vec::new();
        for _ in 0..self.nodes.len() {
            new_row.push(None);
        }

        self.edges.push(new_row);
        index
    }

    fn add_edge(&mut self, edge: E, from: usize, to: usize) {
        if from >= self.nodes.len() || to >= self.nodes.len() {
            panic!("Node index out of bounds");
        }
        self.edges[from][to] = Some(edge);
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edges
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&e| e.is_some())
            .count()
    }
}

struct Counter {
    count: i32,
    limit: i32,
}

impl Counter {
    fn new(limit: i32) -> Self {
        Counter { count: 0, limit }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.limit {
            let result = self.count;
            self.count += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// # 默认泛型参数
/// 在 Rust 中，"RHS" 通常指的是 "right-hand side"，即在表达式中等号右侧的部分。
/// 这个术语通常用于讨论赋值、比较或其他二元操作符的上下文中。
/// 在 Rust 编程中，理解 RHS 是理解表达式求值和类型推断的关键部分。
fn default_generic_params() {
    dbg!(Point { x: 1, y: 0 } + Point { x: 3, y: 3 });
    dbg!(Millimeters(1) + Meters(2));
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 模拟 运算符重载
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

// 两种不同类型相加 操作符重载
impl Add<Meters>  for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
          Millimeters(self.0 + rhs.0 * 1000)
    }
}
// 12_rc_graph_dag.rs

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GraphNode {
    id: usize,
    name: String,
    // Using Rc<RefCell<...>> allows shared ownership and interior mutability of edges.
    // - Rc: A node can be pointed to by multiple other nodes (multiple incoming edges).
    // - RefCell: We can add edges to a node even if it is immutable or shared.
    edges: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    fn new(id: usize, name: &str) -> Rc<Self> {
        Rc::new(GraphNode {
            id,
            name: name.to_string(),
            edges: RefCell::new(Vec::new()),
        })
    }

    // Adds a directed edge from one node to another.
    fn add_edge(from: &Rc<GraphNode>, to: &Rc<GraphNode>) {
        // We borrow the 'edges' list mutably to push a new connection.
        // Rc::clone increments the reference count of 'to', sharing ownership.
        from.edges.borrow_mut().push(Rc::clone(to));
    }

    fn print_connections(&self) {
        print!("Node '{}' (ID {}) connects to: ", self.name, self.id);
        if self.edges.borrow().is_empty() {
            print!("(none)");
        } else {
            for (i, node) in self.edges.borrow().iter().enumerate() {
                if i > 0 { print!(", "); }
                print!("{}", node.name);
            }
        }
        println!();
    }
}

fn main() {
    let node_a = GraphNode::new(1, "A");
    let node_b = GraphNode::new(2, "B");
    let node_c = GraphNode::new(3, "C");
    let node_d = GraphNode::new(4, "D");

    // Create connections for a Directed Acyclic Graph (DAG):
    // A -> B
    // A -> C
    // B -> C
    // C -> D
    GraphNode::add_edge(&node_a, &node_b);
    GraphNode::add_edge(&node_a, &node_c);
    GraphNode::add_edge(&node_b, &node_c);
    GraphNode::add_edge(&node_c, &node_d);

    println!("--- Graph Connections ---");
    node_a.print_connections();
    node_b.print_connections();
    node_c.print_connections();
    node_d.print_connections();
    println!("-------------------------");

    println!("\n--- Reference Counts ---");
    // The strong_count tells us how many things are pointing to a node.
    // It is (1 from main) + (number of incoming edges).

    // Owned by main (1)
    println!("Node A strong_count: {}", Rc::strong_count(&node_a)); 

    // Owned by main (1) + edge from A (1) = 2
    println!("Node B strong_count: {}", Rc::strong_count(&node_b)); 

    // Owned by main (1) + edge from A (1) + edge from B (1) = 3
    println!("Node C strong_count: {}", Rc::strong_count(&node_c)); 

    // Owned by main (1) + edge from C (1) = 2
    println!("Node D strong_count: {}", Rc::strong_count(&node_d)); 
    
    // Note: Since this is a DAG (no cycles), all ref counts will hit 0 
    // when `main` ends, and memory will be cleaned up correctly.
}
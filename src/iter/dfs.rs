use crate::graph_structure::GraphStruct;
use crate::graph_structure::graph::GetKey;
use std::rc::Rc;
use std::collections::HashSet;

#[derive(Clone)]
struct Pair<GraphNode> {
	graph_node: GraphNode,
	ith: usize
}
impl<GraphNode: GetKey + Clone> Pair<GraphNode> {
	fn make_pair(graph_node: GraphNode, ith: usize) -> Self {
		Pair {graph_node, ith}
	}
}
pub struct Iter<GraphNode: GetKey + Clone> {
	pointer_stack: Vec<Pair<GraphNode>>,
	graph_pointer: Rc<GraphStruct<GraphNode>>,
	head_point: GraphNode,
	is_first: bool,
	visited: HashSet<usize>
}

impl<GraphNode: GetKey + Clone> Iter<GraphNode> {
	pub fn new(head_point: GraphNode, graph_pointer: Rc<GraphStruct<GraphNode>>) -> Self {
		Iter { 
			pointer_stack: Vec::new(),
			graph_pointer: graph_pointer.clone(),
			head_point: head_point.clone(),
			is_first: true,
			visited: HashSet::new(),
		}
	}
}

impl<GraphNode: GetKey + Clone> Iterator for Iter<GraphNode> {
	type Item = GraphNode;
	fn next(&mut self) -> Option<GraphNode> {
		if (self.is_first) {
			self.is_first = false;
			let current_pointer:&GraphNode = &self.head_point;
			self.pointer_stack.push(Pair::make_pair(current_pointer.clone(), 0usize));
			self.visited.insert(current_pointer.get_key());
			return Some(current_pointer.clone());
		}
		else {
			if let Some(last_ref) = self.pointer_stack.last_mut() {
				println!("stack end point {}", last_ref.graph_node.get_key());
				if (last_ref.ith == self.graph_pointer.get_neighbor(last_ref.graph_node.get_key()).unwrap().len()) {
					self.pointer_stack.pop();
					self.next()
				} 
				else {
					let point = self.graph_pointer.get_neighbor(last_ref.graph_node.get_key()).unwrap().get(last_ref.ith).unwrap();
					if (self.visited.contains(&point.get_key())) {
						last_ref.ith += 1;
						self.next()
					}
					else {
						last_ref.ith += 1;
						self.pointer_stack.push(Pair::make_pair(point.clone(), 0usize));
						self.visited.insert(point.get_key());
						return Some(point.clone());
					}
				}
			}
			else {
				return None;
			}
		}
	}
}

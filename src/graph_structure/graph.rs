use std::vec::Vec;
pub trait GetKey {
	fn get_key(&self) ->usize;
}

impl GetKey for u32 {
	fn get_key(&self) -> usize { 
		*self as usize
	}
}
pub struct GraphStruct<GraphNode: GetKey + Clone>{
	edge_list: Vec<Vec<GraphNode>>
}

impl<GraphNode:GetKey + Clone> GraphStruct<GraphNode> {
	pub fn new() -> Self {
		Self {
			edge_list: Vec::new(),
		}
	}
	pub fn AppendEdgeList(&self, index: usize, edges: &Vec<GraphNode>) -> Self {
		let mut edge_list = self.edge_list.clone();
		while edge_list.len() < index + 1 {
			edge_list.push(vec![]);
		}
		edge_list[index] = edges.clone();
		Self { edge_list: edge_list}
	}
	pub fn AddEdgeList(&mut self, index: usize, edges: &Vec<GraphNode>) {
		while self.edge_list.len() < index + 1 {
			self.edge_list.push(vec![]);
		}
		self.edge_list[index] = edges.clone();
	}
	pub fn get_neighbor(&self, index: usize) -> Option<&Vec<GraphNode>> {
		self.edge_list.get(index)
	}

	// pub fn iter(&self) -> Iter<GraphNode> {

	// }
}

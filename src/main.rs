use iter_dfs::graph_structure;
use std::rc::Rc;
use std::cell::RefCell;
use iter_dfs::iter::dfs::Iter;

fn main() {
	println!("hello");
	let mut graph = Rc::new(graph_structure::GraphStruct::<u32>::new());
	graph = Rc::new(graph.AppendEdgeList(0, &vec![1,2]));
	graph = Rc::new(graph.AppendEdgeList(1, &vec![2]));
	graph = Rc::new(graph.AppendEdgeList(2, &vec![]));
	
	let mut graph_iter = Iter::new(0, graph.clone());
	let x = graph_iter.next().unwrap();
	assert_eq!(x, 0);
	let y = graph_iter.next().unwrap();
	assert_eq!(y, 1);
	let z = graph_iter.next().unwrap();
	assert_eq!(z, 2);
	let end = graph_iter.next();
	assert_eq!(end, None);
}
use std::env;
use std::fs::File;

#[allow(non_snake_case)]
fn generateGraph(filename: String) -> Graph {
	//let mut G = Graph{nodes: vec![]};
	//G
}

#[allow(non_snake_case)]
fn main()
{
	let fname = env::args().nth(2).unwrap();
	let G = generateGraph(fname);
	//G.nodes.push(String::from("hello"));
	//G.edges.push(Edge(String::from("hello"),String::from("goodbye"), 1));
	//for e in G.edges
	//{
	//	println!("{} -> {} with weight {}", e.0,e.1,e.2);
	//}
}
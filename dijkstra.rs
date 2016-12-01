use std::env;

struct Edge(String, String, i32);

struct Graph {
	nodes: Vec<String>,
	edges: Vec<Edge>, 
}

#[allow(non_snake_case)]
fn generateGraph(filename: String) -> Graph {
	let mut G = Graph{nodes: vec![],edges: vec![]};
	G
}


fn main()
{
	let fname = env::args().nth(2).unwrap();
	println!("{}", fname);
	#[allow(non_snake_case)]
	let G = generateGraph(fname);
	//G.nodes.push(String::from("hello"));
	//G.edges.push(Edge(String::from("hello"),String::from("goodbye"), 1));
	//for e in G.edges
	//{
	//	println!("{} -> {} with weight {}", e.0,e.1,e.2);
	//}

}
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
	#[allow(non_snake_case)]
	let mut count = 0;
	let mut fname = String::new();
	for arg in env::args()
	{
		if count == 2
		{
			fname = arg;
		}
		count += 1;
	}
	println!("{}", fname);
	let G = generateGraph(fname);
	//G.nodes.push(String::from("hello"));
	//G.edges.push(Edge(String::from("hello"),String::from("goodbye"), 1));
	//for e in G.edges
	//{
	//	println!("{} -> {} with weight {}", e.0,e.1,e.2);
	//}

}
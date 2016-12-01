
struct Edge(String, String, i32);

struct Graph {
	nodes: Vec<String>,
	edges: Vec<Edge>, 
}


fn main()
{
	let mut G = Graph{nodes: vec![],edges: vec![]};
	G.nodes.push("hello");
	G.edges.push(("hello", "goodbye", 1));
	for e in G.edges
	{
		println!("{} -> {} with weight {}", e.0,e.1,e.2);
	}
}
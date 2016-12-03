extern crate petgraph;

use petgraph::graphmap::UnGraphMap;

#[allow(non_snake_case)]
fn generate_graph<'a>(lines: Vec<&'static str>) -> UnGraphMap<&'static str, i32> 
{
	let mut G: UnGraphMap<&str,i32> = UnGraphMap::new();
	for line in lines
	{
		let vals: Vec<&str> = line.split('|').collect();
		let weight = match vals[2].parse::<i32>()
  		{		  		
 			Ok(weight) => weight,
 			Err(_) => panic!("can't be converted to i32"),
		};
		G.add_edge(vals[0],vals[1],weight);
	}
	G
}

#[allow(non_snake_case)]
fn dijkstra(G: UnGraphMap<&str, i32>,source: &str) -> String
{
	let mut path = String::new();
	let mut nodes: Vec<&str> = Vec::new();

	for n in nodes
	{
		path += n;
	}
	return path;
}

fn def_lines<'a>() -> Vec<&'a str>
{
	let mut lines: Vec<&'a str> = Vec::new();
	lines.push("A|B|1");
	lines.push("A|D|3");
	lines.push("A|C|1");
	lines.push("C|D|1");
	lines.push("B|D|4");
	lines
}

#[allow(non_snake_case)]
fn main()
{
	let G = generate_graph(def_lines());
	println!("{}", G.node_count());
	let path = dijkstra(G, "A");
	println!("Shortest path is: {}", path);
}
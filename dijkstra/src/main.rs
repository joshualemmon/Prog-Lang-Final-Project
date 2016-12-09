extern crate petgraph;

use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use std::collections::HashSet;

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
	let mut dist: HashMap<&str, i32> = HashMap::new();
	let mut parent: HashMap<&str, &str> = HashMap::new();
	let mut S: HashSet<&str> = HashSet::new();
	let mut sorted: Vec<&str> = Vec::new();
	for v in G.nodes()
	{
		dist.insert(v,i32::max_value());
		parent.insert(v,v);
		S.insert(v);
		sorted.push(v);
	}
	dist.insert(source, 0);

	let mut swapped = true;
	let n = sorted.len();
	while swapped == true
	{
		swapped = false;
		for i in 1..(n-1)
		{
			if dist[sorted[i-1]] > dist[sorted[i]]
			{
				let s = sorted[i];
				sorted[i] = sorted[i-1];
				sorted[i-1] = s;
				swapped = true;
			}
		}
	}
	let mut i = 0;

	while S.is_empty() == false
	{
		let u = sorted[i];
		i+=1;
		S.remove(u);
		for v in G.neighbors(u)
		{
			let edge = G.edge_weight(u,v);
			/*let e = match edge
			{
				Some(ref val) => *val,
				None => -1,
			};*/
			let e = edge.unwrap();

			let du = dist[u];
			if dist[v] > du + e
			{
				dist.insert(v,(du + e));
				parent.insert(v,u);
			}
		}
	}
	for n in G.nodes()
	{
		println!("{} : {}",n,dist[n]);
	}

	return String::new();
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
	//println!("{}", G.node_count());
	let path = dijkstra(G, "A");
	println!("Shortest path is: {}", path);
}
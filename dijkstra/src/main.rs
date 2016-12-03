extern crate petgraph;

use std::env;
use std::fs::File;
use petgraph::Graph;
use std::io::BufReader;
use std::io::BufRead;
use std::str::Split;

#[allow(non_snake_case)]
fn generateGraph(filename: String) -> Graph<String, i32> 
{
	let mut G = Graph::<String, i32>::new();
	let file = match File::open(filename) 
	{
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
	let reader = BufReader::new(&file);
	//let lines: Vec<_> = reader.lines().collect();
	for l in reader.lines()
	{
		let line = l.unwrap();
		let vals: Vec<&str> = line.split('|').collect();
		let n1 = G.add_node(String::from(vals[0]));
		let n2 = G.add_node(String::from(vals[1]));
		let weight = match vals[2].parse::<i32>()
		{
			Ok(weight) => weight,
			Err(_) => panic!("can't be converted to i32"),
		};
		G.add_edge(n1,n2,weight);
	}
	G
}

fn dijkstra(G: Graph<String, i32>,source: String) -> String
{
	let path = String::new();
	return path;
}

#[allow(non_snake_case)]
fn main()
{
	let fname = String::from("vals.txt");
	let G = generateGraph(fname);
	println!("{}",G.node_count());
	//let path = dijkstra(G, "A");
}
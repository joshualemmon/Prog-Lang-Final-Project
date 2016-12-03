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
		G.add_node(String::from(vals[0]));
		G.add_node(String::from(vals[1]));
		let weight = match vals[2].parse::<i32>()
		{
			Ok(weight) => weight,
			Err(_) => panic!("can't be converted to i32"),
		};
		G.add_edge(String::from(vals[0]),String::from(vals[1]),weight);
	}
	return G;
}

#[allow(non_snake_case)]
fn main()
{
	let fname = String::from("vals.txt");
	let G = generateGraph(fname);
	//G.nodes.push(String::from("hello"));
	//G.edges.push(Edge(String::from("hello"),String::from("goodbye"), 1));
	//for e in G.edges
	//{
	//	println!("{} -> {} with weight {}", e.0,e.1,e.2);
	//}
}
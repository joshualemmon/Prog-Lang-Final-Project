extern crate petgraph;

use std::env;
use std::fs::File;
use petgraph::Graph;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;

#[allow(non_snake_case)]
fn generateGraph(filename: String) -> Graph<String, i32> 
{
	let mut G = Graph::<String, i32>::new();
	let mut file = match File::open(filename) 
	{
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
	let reader = BufReader::new(file);
	let lines: Vec<_> = reader.lines().collect();
	for l in lines 
	{
		let vals: Vec<_> = l.split("|").collect();
		for v in vals
		{
			println!("{}", v);
		} 
	}
	return G;
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
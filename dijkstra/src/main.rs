extern crate petgraph;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;
use petgraph::graphmap::GraphMap;

#[allow(non_snake_case)]
fn generateGraph(filename: String) -> () 
{
	let mut G: UnGraphMap<&str,i32> = UnGraphMap::new();
	let file = match File::open(filename) 
	{
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
	let reader = BufReader::new(&file);
	let mut vals: Vec<String> = Vec::new();
	for l in reader.lines()
	{
		let line = l.unwrap();
		vals.push(line);
	}
	for val in vals
	{
		let vals: Vec<&str> = val.split('|').collect();
		let weight = match vals[2].parse::<i32>()
  		{		  		
 			Ok(weight) => weight,
 			Err(_) => panic!("can't be converted to i32"),
		};
		G.add_edge(vals[0],vals[1],weight);
	}
	dijkstra(G, "A");
}

fn dijkstra(G: UnGraphMap<&'static str, i32>,source: &str) -> String
{
	let path = String::new();
	return path;
}

#[allow(non_snake_case)]
fn main()
{
	let fname = String::from("vals.txt");
	generateGraph(fname);
	//println!("{}",G.node_count());
	//let path = dijkstra(G, "A");
}
extern crate petgraph;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;
use petgraph::graphmap::GraphMap;

#[allow(non_snake_case)]
fn generateGraph(filename: &str) -> UnGraphMap<&str, i32> 
{
	let mut G: UnGraphMap<&str,i32> = UnGraphMap::new();
	let file = match File::open(filename) 
	{
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
	let reader = BufReader::new(&file);
	let mut lines: Vec<&'static str> = Vec::new();
	for l in reader.lines()
	{
		let line: String = l.unwrap();
		let s: &'static str = &*line; 
		lines.push(s);
	}
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

fn dijkstra(G: UnGraphMap<&str, i32>,source: &str) -> String
{
	let path = String::new();
	return path;
}

#[allow(non_snake_case)]
fn main()
{
	let G = generateGraph("vals.txt");
	//println!("{}",G.node_count());
	//let path = dijkstra(G, "A");
}
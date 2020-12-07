use petgraph::prelude::Dfs;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::prelude::GraphMap;
use petgraph::Graph;
use std::collections::HashSet;
use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1_example() {
        assert_eq!(day7_part1("data/day7/example"), 4);
    }

    #[test]
    fn test_day7_part1() {
        assert_eq!(day7_part1("data/day7/input"), 144);
    }


}

pub fn day7_part1(path: &str) -> usize {
    let mut bag_graph = Graph::<&str, u32>::new();

    let mut node_names = HashSet::<String>::new();

    read(path).for_each(|l| {
        let (node, connections) = line_to_bags(&l);
        
        node_names.insert(node.to_string());
        for c in connections {
            node_names.insert(c);
        }
    }); 

    let map: HashMap<String, NodeIndex> = node_names.iter().map(|s| (s.to_string(), bag_graph.add_node(s))).collect::<HashMap<String, NodeIndex>>();

    read(path).for_each(|l| {
        let (node, connections) = line_to_bags(&l);
        let node_i = map.get(&node);
        
        for c in connections {
            
            bag_graph.add_edge(*map.get(&c).unwrap(), *node_i.unwrap(), 1);
        }
    });

    println!("{:?}", bag_graph);

    let start = map.get("shiny gold ").unwrap();
    let mut dfs = Dfs::new(&bag_graph, *start);
    let mut count = 0;

    while let Some(visited) = dfs.next(&bag_graph) {
        println!("v {}", visited.index());
        if map.get("other bags. ").unwrap().index() != visited.index() {
            count += 1;
        }
    }

    // to correct for the node itself and for
    count - 1
}

fn line_to_bags(line: &str) -> (String, HashSet<String>) {


    let first = line.split(',').nth(0).unwrap();
    
    let mut connections = HashSet::new();

    let mut target = String::new();
    first.split(' ').take_while(|&w| w != "bags").for_each(|w| {
        target.push_str(w);
        target.push_str(" ");
    });


    let mut first_content = String::new(); 
    first.split(' ').skip(5).take(2).for_each(|w| {
        first_content.push_str(w);
        first_content.push_str(" ");
    });

    connections.insert(first_content);

    for sub_seq_contents in line.split(',').skip(1) {
        let mut sc = String::new();
        sub_seq_contents.split(' ').skip(2).take(2).for_each(|w| {
            sc.push_str(w);
            sc.push_str(" ");
        });
        connections.insert(sc);
    }

    (target, connections)  
}


use petgraph::Outgoing;
use petgraph::prelude::Dfs;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::graph::EdgeIndex;
use petgraph::graph::EdgeReference;
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

    #[test]
    fn test_day7_part2_example() {
        assert_eq!(day7_part2("data/day7/example2"), 126);
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!(day7_part2("data/day7/input"), 126);
    }


}

pub fn day7_part1(path: &str) -> usize {
    let mut bag_graph = Graph::<&str, u32>::new();

    let mut node_names = HashSet::<String>::new();

    read(path).for_each(|l| {
        let (node, connections) = line_to_bags(&l);
        
        node_names.insert(node.to_string());
        for c in connections {
            node_names.insert(c.0);
        }
    }); 

    let map: HashMap<String, NodeIndex> = node_names.iter().map(|s| (s.to_string(), bag_graph.add_node(s))).collect::<HashMap<String, NodeIndex>>();

    read(path).for_each(|l| {

        let (node, connections) = line_to_bags(&l);
        let node_i = map.get(&node);
        
        for c in connections {
            println!("{:?}", c);    
            bag_graph.add_edge(*map.get(&c.0).unwrap(), *node_i.unwrap(), c.1 as u32);
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

    for n in bag_graph.edges(*start) {
        println!("{:?}", n);
    }
    // to correct for the node itself and for
    count - 1
}

pub fn day7_part2(path: &str) -> usize {
    let mut bag_graph = Graph::<&str, u32>::new();

    let mut node_names = HashSet::<String>::new();

    read(path).for_each(|l| {
        let (node, connections) = line_to_bags(&l);
        
        node_names.insert(node.to_string());
        for c in connections {
            node_names.insert(c.0);
        }
    }); 

    let map: HashMap<String, NodeIndex> = node_names.iter().map(|s| (s.to_string(), bag_graph.add_node(s))).collect::<HashMap<String, NodeIndex>>();

    read(path).for_each(|l| {

        let (node, connections) = line_to_bags(&l);
        let node_i = map.get(&node);
        
        for c in connections {
            println!("{:?}", c);    
            bag_graph.add_edge(*node_i.unwrap(), *map.get(&c.0).unwrap(), c.1 as u32);
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

    bag_graph.neighbors_directed(*start, Outgoing).for_each(|n| {
        let e = bag_graph.find_edge(*start, n).unwrap();
        println!("{:?}", bag_graph.edge_weight(e).unwrap());
        println!("{:?}", n);
    });

    // to correct for the node itself and for
    calc_num_bags(&bag_graph, *start)
}

fn calc_num_bags(graph: &Graph::<&str, u32>, start: NodeIndex) -> usize {

    if graph.neighbors(start).count() == 0 {
        1
    } else {
        graph.neighbors_directed(start, Outgoing).map(|n| {
            let e = graph.find_edge(start, n).unwrap();
            let ew = graph.edge_weight(e).unwrap();
            println!("ew {}", ew);
            //let new_count = *ew as usize + count * *ew as usize;

            *ew as usize + *ew as usize * calc_num_bags(graph, n)
        }).sum()
    }
}

fn line_to_bags(line: &str) -> (String, HashSet<(String, usize)>) {


    let first = line.split(',').nth(0).unwrap();
    
    let mut connections = HashSet::new();

    let mut target = String::new();
    first.split(' ').take_while(|&w| w != "bags").for_each(|w| {
        target.push_str(w);
        target.push_str(" ");
    });


    let mut first_content = String::new(); 
    println!("{}", line);
    let first_weight = first.split(' ').skip(4).take(1).nth(0).unwrap();
    let first_weight = match first_weight {
        "no" => 0,
        _ => first_weight.parse().unwrap()
    };

    first.split(' ').skip(5).take(2).for_each(|w| {
        first_content.push_str(w);
        first_content.push_str(" ");
    });

    connections.insert((first_content, first_weight));

    for sub_seq_contents in line.split(',').skip(1) {
        let mut sc = String::new();
        sub_seq_contents.split(' ').skip(2).take(2).for_each(|w| {
            sc.push_str(w);
            sc.push_str(" ");
        });

        let sc_weight = sub_seq_contents.split(' ').skip(1).take(1).nth(0).unwrap().parse().unwrap();
        connections.insert((sc, sc_weight));
    }

    (target, connections)  
}


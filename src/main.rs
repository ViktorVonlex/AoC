use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut t_names: HashSet<String> = HashSet::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            let mut parts = line.split('-');
            let one = parts.next().unwrap().to_string();
            let two = parts.next().unwrap().to_string();

            connections
                .entry(one.clone())
                .or_insert_with(HashSet::new)
                .insert(two.clone());
            connections
                .entry(two.clone())
                .or_insert_with(HashSet::new)
                .insert(one.clone());

            if one.starts_with('t') {
                t_names.insert(one);
            }
            if two.starts_with('t') {
                t_names.insert(two);
            }
        }
    }

    let mut unique_triplets: HashSet<Vec<String>> = HashSet::new();
    find_triplets(&t_names, &connections, &mut unique_triplets);
    println!("Number of unique triplets: {}", unique_triplets.len());

    let triplets: Vec<Vec<String>> = unique_triplets.into_iter().collect();

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for triplet in triplets {
        let (a, b, c) = (&triplet[0], &triplet[1], &triplet[2]);
        graph
            .entry(a.clone())
            .or_insert_with(HashSet::new)
            .extend([b.clone(), c.clone()]);
        graph
            .entry(b.clone())
            .or_insert_with(HashSet::new)
            .extend([a.clone(), c.clone()]);
        graph
            .entry(c.clone())
            .or_insert_with(HashSet::new)
            .extend([a.clone(), b.clone()]);
    }

    let largest_clique = find_largest_clique(&graph);
    let mut password = largest_clique.clone();
    password.sort();
    println!("Password for the LAN party: {}", password.join(","));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_triplets(
    t_names: &HashSet<String>,
    connections: &HashMap<String, HashSet<String>>,
    unique_triplets: &mut HashSet<Vec<String>>,
) {
    for node in connections.keys() {
        if let Some(neighbors) = connections.get(node) {
            for a in neighbors {
                for b in neighbors {
                    if a < b
                        && connections
                            .get(a)
                            .map_or(false, |a_neighbors| a_neighbors.contains(b))
                    {
                        let mut triplet = vec![node.clone(), a.clone(), b.clone()];
                        triplet.sort();
                        if triplet.iter().any(|name| t_names.contains(name)) {
                            unique_triplets.insert(triplet);
                        }
                    }
                }
            }
        }
    }
}

fn find_largest_clique(graph: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut largest_clique = Vec::new();

    for (node, neighbors) in graph {
        let mut clique = vec![node.clone()];
        for neighbor in neighbors {
            if clique
                .iter()
                .all(|member| graph.get(member).unwrap().contains(neighbor))
            {
                clique.push(neighbor.clone());
            }
        }

        if clique.len() > largest_clique.len() {
            largest_clique = clique;
        }
    }

    largest_clique
}

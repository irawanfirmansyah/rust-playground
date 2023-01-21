use std::collections::HashMap;

/**
 * Problem: Given graph with key K as node and value V as list of dependencies.
 *          Return list of root nodes (not having dependencies)
 * ex:      Graph:
 *          A --> B --> C
 *          D --> E
 *          
 *          Ans:
 *          ["C", "E"]
 * 
 * 
 */
pub fn solution_3(monorepo_graph: &HashMap<&str, Vec<&str>>) -> Vec<String> {
  let mut map: HashMap<String, bool> = HashMap::new();

  // println!("{:?}", map);
  // Assume all nodes are root (no deps)
  for (package_1, _) in monorepo_graph.iter() {
    map.insert(package_1.to_string(), true);
  }
  println!("{:?}", map);

  for (package, dependencies) in monorepo_graph.iter() {
    if dependencies.len() > 0 {
      map.remove(*package);
    }
  }

  return map.keys().cloned().collect();
}

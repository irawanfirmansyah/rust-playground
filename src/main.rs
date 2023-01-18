use std::{collections::HashMap, vec};

/**
 * Problem:
 * given list of integers, return the biggest n integer in the list which has n occurrence as well in the list
 * ex:
 * - [4, 10, 2, 4, 5, 6, 2, 4] -> return 2. num 2 has 2 occurrence in the array
 * - [6, 2, 3, 1, 4] -> return 1. 1 has exactly 1 occurence in the array and no.
 */
fn solution(list: Vec<i32>) -> i32 {
  let mut nums_count: HashMap<i32, i32> = HashMap::new();
  let mut v: Vec<i32> = Vec::new();

  for n in list {
    if nums_count.contains_key(&n) {
      nums_count.insert(n, nums_count.get(&n).unwrap() + 1);
    } else {
      nums_count.insert(n, 1);
    }

    if !v.last().is_none() {
      let last = *v.last().unwrap();
      let num_count = *nums_count.get(&n).unwrap();

      if last == n {
        if n < num_count {
          v.pop();
        }
      }

      if last < n {
        if n == num_count {
          v.push(n);
        }
      }
    } else {
      let num_count = *nums_count.get(&n).unwrap();
      if n == num_count {
        v.push(n);
      }
    }
  }

  return *v.last().unwrap_or(&0);
}

/**
 * Problem:
 * given an array, return min number of move needed to
 * avoid negative accumulation (from left to right)
 * ex:
 * - [1, -2, 3, -4, 3, 2] -> return 1. only need to move -2 to the end
 * - [1, -11, -2, 9, 8] -> return 2. need to move -11 and -2 to the end
 *
 */
fn solution_2(list: Vec<i32>) -> i32 {
  let mut v: Vec<i32> = Vec::new();
  let mut mov = 0;
  let mut sum = 0;
  v.extend_from_slice(&list);

  while !v.is_empty() {
    let curr = v.first().unwrap();

    if sum + curr < 0 {
      mov += 1;
    } else {
      sum += curr;
    }
    v.remove(0);
  }

  return mov;
}

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
fn solution_3(monorepo_graph: &HashMap<&str, Vec<&str>>) -> Vec<String> {
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

fn read_value(i: &i8) {
  // Use immutable reference 'i':
  println!("num = {}", *i);
}

fn change_value(i: &mut i8) {
  // Use mutable reference 'i':
  *i = 5;
}

fn main() {
  println!("---- SOLUTION 1 ----");
  println!("{}", solution(vec![4, 10, 2, 4, 5, 6, 2, 4]));
  println!("{}", solution(vec![6, 2, 3, 1, 4]));
  println!("{}", solution(vec![]));
  println!(
    "{}",
    solution(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5])
  );

  println!("---- SOLUTION 2 ----");
  println!("{}", solution_2(vec![5, 1, -8, 9, 2, -7, 3, 3]));
  println!("{}", solution_2(vec![1, -2, 3, -4, 3, 2]));
  println!("{}", solution_2(vec![1, -11, -2, 9, 8]));
  println!("{}", solution_2(vec![1, -11, -2, 9, 8]));

  let graph_1 = HashMap::from([
    ("A", vec!["B"]),
    ("B", vec!["C"]),
    ("C", vec![]),
    ("D", vec!["E"]),
    ("E", vec![]),
  ]);

  println!("{:?}", solution_3(&graph_1));

  let mut x = 10;

  let ref1 = &x;
  read_value(ref1); // Lifetime of ref1 ends here

  let ref2 = &mut x;
  change_value(ref2);

  // Print new value of 'x':
  println!("New value of x = {}", *ref2);
}

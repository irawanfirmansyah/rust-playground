use std::collections::HashMap;

fn solution(list: Vec<i32>) -> i32 {
  let mut nums_count: HashMap<i32, i32> = HashMap::new();

  for n in list {
    if nums_count.contains_key(&n) {
      *nums_count.get_mut(&n).unwrap() = *nums_count.get_mut(&n).unwrap() + 1
    } else {
      nums_count.insert(n, 1);
    }
  }

  let mut result = 0;

  for (num, num_count) in &nums_count {
    if num == num_count && &result < num {
      result = *num;
    }
  }

  return result;
}

fn solution_2(list: Vec<i32>, sum: i32, count_move: i32) -> i32 {
  println!("list: {:?}, sum: {}, move: {}", list, sum, count_move);
  if list.len() <= 0 {
    return count_move;
  }

  let curr = list.first().unwrap();

  let mut new_list = Vec::new();

  if sum + *curr < 0 {
    new_list.extend_from_slice(&list[1..]);
    new_list.push(*curr);

    return solution_2(new_list, sum, count_move + 1);
  }

  new_list.extend_from_slice(&list[1..]);

  return solution_2(new_list, sum + *curr, count_move);
}

fn read_value(i: &i8) {
  // Use immutable reference 'i':
  println!("x = {}", *i);
}

fn change_value(i: &mut i8) {
  // Use mutable reference 'i':
  *i = 5;
}

fn main() {
  solution(vec![5, 1, 8, 9, 2, 2, 3, 7, 3, 3]);
  // println!("{}", solution(inp1));

  println!("{}", solution_2(vec![5, 1, -8, 9, 2, -7, 3, 3], 0, 0));

  let mut x = 10;

  let ref1 = &x;
  read_value(ref1); // Lifetime of ref1 ends here

  let ref2 = &mut x;
  change_value(ref2);

  // Print new value of 'x':
  // println!("New value of x = {}", *ref2);
}

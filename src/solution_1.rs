use std::collections::HashMap;

/**
 * Problem:
 * given list of integers, return the biggest n integer in the list which has n occurrence as well in the list
 * ex:
 * - [4, 10, 2, 4, 5, 6, 2, 4] -> return 2. num 2 has 2 occurrence in the array
 * - [6, 2, 3, 1, 4] -> return 1. 1 has exactly 1 occurence in the array and no.
 */
pub fn solution(list: Vec<i32>) -> i32 {
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

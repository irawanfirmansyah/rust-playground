/**
 * Problem:
 * given an array, return min number of move needed to
 * avoid negative accumulation (from left to right)
 * ex:
 * - [1, -2, 3, -4, 3, 2] -> return 1. only need to move -2 to the end
 * - [1, -11, -2, 9, 8] -> return 2. need to move -11 and -2 to the end
 *
 */
pub fn solution_2(list: Vec<i32>) -> i32 {
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
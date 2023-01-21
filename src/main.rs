mod fibonacci;
mod solution_1;
mod solution_2;

fn read_value(i: &i8) {
  // Use immutable reference 'i':
  println!("num = {}", *i);
}

fn change_value(i: &mut i8) {
  // Use mutable reference 'i':
  *i = 5;
}

fn main() {
  let result = fibonacci::fibo_3(10);
  println!("{}", result);

  println!("---- SOLUTION 1 ----");
  println!("{}", solution_1::solution(vec![4, 10, 2, 4, 5, 6, 2, 4]));
  println!("{}", solution_1::solution(vec![6, 2, 3, 1, 4]));
  println!("{}", solution_1::solution(vec![]));
  println!(
    "{}",
    solution_1::solution(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5])
  );

  println!("---- SOLUTION 2 ----");
  println!("{}", solution_2::solution_2(vec![5, 1, -8, 9, 2, -7, 3, 3]));
  println!("{}", solution_2::solution_2(vec![1, -2, 3, -4, 3, 2]));
  println!("{}", solution_2::solution_2(vec![1, -11, -2, 9, 8]));
  println!("{}", solution_2::solution_2(vec![1, -11, -2, 9, 8]));

  let mut x = 10;

  let ref1 = &x;
  read_value(ref1); // Lifetime of ref1 ends here

  let ref2 = &mut x;
  change_value(ref2);

  // Print new value of 'x':
  println!("New value of x = {}", *ref2);
}

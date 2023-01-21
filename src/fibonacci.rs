pub fn fibo_3(n: i128) -> i128 {
  let mut tup = [0, 1];
  if n <= 0 {
    return tup[0];
  }

  if n == 1 {
    return tup[1];
  }

  for _ in 2..n {
    println!("{:?}", tup);
    let next_value = tup[0] + tup[1];
    tup[0] = tup[1];
    tup[1] = next_value
  }

  return tup[0] + tup[1];
}

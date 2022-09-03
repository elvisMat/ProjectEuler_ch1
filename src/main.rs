fn  problem1(num: usize) -> usize {
  let mut sum: usize = 0;
  for t in 1..num + 1 {
    if t % 3 == 0 || t % 5 == 0 {
     // println!("{}", t);
      sum += t;
    }
  }
  sum
}
fn main() {
  //testing
  assert_eq!(23, problem1(9));
  assert_eq!(3, problem1(3));
  assert_eq!(14, problem1(7));
  assert_eq!(233168, problem1(999));
}

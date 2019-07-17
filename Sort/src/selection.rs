fn main() {
  let mut array =  [32, 1, 45, 900, 2000, 3, 5, 10, 0];
  sort(&mut array);
  println!("{:?}", array);
}

fn sort(A: &mut [u32]) {
  for i in 0..A.len() {
    let mut min = i;
    for j in i + 1..A.len() {
      if A[j] < A[min] {
        min = j;
      }
    }
    A.swap(i, min);
  }
}
fn main() {
  let mut array =  [32, 1, 45, 900, 2000, 3, 5, 10, 0];
  sort(&mut array);
  println!("{:?}", array);
}

fn sort(A: &mut [u32]) {
  for j in 1..A.len() {
    let key = A[j];
    let mut i = j;
    while i > 0 && A[i - 1] > key {
      A[i] = A[i - 1];
      i = i - 1;
    }
    A[i] = key;
  }
}
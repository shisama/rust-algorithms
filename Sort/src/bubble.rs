fn main() {
  let mut array = [32, 1, 45, 900, 2000, 3, 5, 10, 0];
  sort(&mut array);
  println!("{:?}", array);
}

fn sort(A: &mut [u32]) {
  let len = A.len();
  for i in 0..len - 1 {
    for j in (i + 1..len).rev() {
      if A[j - 1] > A[j] {
        let tmp = A[j - 1];
        A[j - 1] = A[j];
        A[j] = tmp;
      }
    }
  }
}
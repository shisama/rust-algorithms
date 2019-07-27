fn main() {
  let A: Vec<u32> = vec![32, 1, 45, 90, 20, 3, 5, 10, 0];
  let mut B = vec![0; A.len()];
  let k = 91;
  sort(A, &mut B, k);
  println!("{:?}", B);
}

fn sort(A: Vec<u32>, B: &mut Vec<u32>, k: usize) {
  let mut C = vec![0; k];
  for j in 0..A.len() - 1 {
    C[A[j] as usize] = C[A[j] as usize] + 1;
  }
  for i in 1..k {
    C[i] = C[i] + C[i - 1];
  }
  for j in (0..A.len()).rev() {
    // println!("{:?}", C[A[j] as usize]);
    B[C[A[j] as usize] as usize] = A[j];
    // println!("{:?}", B);
    C[A[j] as usize] = C[A[j] as usize] - 1;
  }
}
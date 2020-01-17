fn main() {
  let mut array = [32, 1, 45, 900, 2000, 3, 5, 10, 0];
  sort(&mut array);
  println!("{:?}", array);
}

fn sort(a: &mut [u32]) {
  for i in 0..a.len() - 1 {
    for j in (i + 1..a.len()).rev() {
      if a[j - 1] > a[j] {
        a.swap(j - 1, j);
      }
    }
  }
}
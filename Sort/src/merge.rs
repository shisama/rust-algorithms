fn main() {
  let array = [32, 1, 45, 900, 2000, 3, 5, 10, 0];
  //_sort(&mut array);
  let mut a = array;
  sort(&mut a, 0, (array.len() - 1) as usize);
  println!("{:?}", a);
}
fn sort(a: &mut [u32], p: usize, r: usize) {
  if p >= r {
    return;
  }
  let q: usize = (p + r) / 2;
  sort(a, p, q);
  sort(a, q + 1, r);
  merge(a, p, q, r);
}

fn merge(a: &mut [u32], p: usize, q: usize, r: usize) {
  let n1 = q - p + 1;
  let n2 = r - q;
  let mut pl: Vec<u32> = Vec::new();
  let mut pr: Vec<u32> = Vec::new();
  for i in 0..n1 {
    pl.push(a[p + i]);
  }
  for i in 0..n2 {
    pr.push(a[q + i + 1]);
  }
  pl.push(u32::max_value());
  pr.push(u32::max_value());
  let mut i = 0;
  let mut j = 0;
  for k in p..=r {
    if pl[i] <= pr[j] {
      a[k] = pl[i];
      i = i + 1;
    } else {
      a[k] = pr[j];
      j = j + 1;
    }
  }
}
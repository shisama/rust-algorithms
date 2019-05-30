fn main() {
  let mut array = [32, 1, 45, 900, 2000, 3, 5, 10, 0];
  sort(&mut array);
  println!("{:?}", array);
}

fn sort<T: Ord>(a: &mut [T]) {
  _sort(a, 0, (a.len() - 1) as isize);
}

fn _sort<T: Ord>(a: &mut [T], l: isize, r: isize) {
  if l >= r {
    return;
  }
  let mut pl = l;
  let mut pr = r - 1;
  let pivot = r as usize;

  while pl < pr {
      while a[pl as usize] < a[pivot] {
        pl += 1;
      }
      while pr >=0 && a[pr as usize] > a[pivot] {
        pr -= 1;
      }
      if pl < pr {
        a.swap(pl as usize, pr as usize);
      }
  }
  a.swap(pl as usize, pivot);
  _sort(a, l, pl - 1);
  _sort(a, pl + 1, r);
}
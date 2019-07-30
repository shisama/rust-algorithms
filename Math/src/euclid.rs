fn main() {
    let (g, i, j) = euclid(30, 18);
    assert_eq!(g, 6);
    assert_eq!(i, -1);
    assert_eq!(j, 2);
}

fn euclid(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, _i, _j) = euclid(b, a % b);
    let i = _j;
    let j = _i - (a / b) * _j;
    return (g, i, j);
}
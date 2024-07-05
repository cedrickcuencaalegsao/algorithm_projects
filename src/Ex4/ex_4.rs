fn main() {

    let mut x = 1;
    let mut s = 5;

    while x <= 100 {
        let y = x * x;
        s += y;
        x += 1;
    }

    println!("Sum (S): {}", s);
}

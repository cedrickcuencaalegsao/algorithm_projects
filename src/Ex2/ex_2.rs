fn main() {
    
    let mut a = 2;
    let mut b = 3;

    let temp = a;
    a = b;
    b = temp;

    println!("After interchange:");
    println!("A = {}", a);
    println!("B = {}", b);
}

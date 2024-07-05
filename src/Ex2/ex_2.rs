fn main() {
    // Ex 2: Develop an algorithm to interchange the values assigned to two
    // variables A and B. (For example, if A=2 and B=3, after interchange,
    // it should be A=3 and B=2).
    // Algorithm: To interchange the values.
    // 1. [Initialize the variables]
    // A = 2
    // B = 3
    // 2. [Peform the operations]
    // TEMP = A
    // A = B
    // B = TEMP
    // 3. [Print the result]
    // Print A, B
    // 4. [Finished]
    // Stop

    // Initialize the variables
    let mut a = 2;
    let mut b = 3;

    // Perform the interchange
    let temp = a;
    a = b;
    b = temp;

    // Print the result
    println!("After interchange:");
    println!("A = {}", a);
    println!("B = {}", b);
}

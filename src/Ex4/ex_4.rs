fn main() {
    // Ex 4: Develop an algorithm to evaluate S using the relation:
    // S=1 + 4 + 9+ 16 + …..+ 100

    // Algorithm:
    // 1. [Initialize the variables]
    // X=1
    // S=0
    // 2. [Perform the operations]
    // Y = X*X
    // S=S + Y
    // X=X+1
    // 3. [Check the condition]
    // If (X  100) Then
    // Goto step 2
    // End if
    // 4. [Print the output]
    // Print S
    // 5. [Finished]
    // Stop

    let mut x = 1;
    let mut s = 5;

    while x <= 100 {
        let y = x * x;
        s += y;
        x += 1;
    }

    println!("Sum (S): {}", s);
}

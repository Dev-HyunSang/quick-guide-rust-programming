fn main() {
    let num = 5;
    let var = if num % 3 == 0 {
        3 // 블럭의 값임.
    } else {
        if num % 5 == 0 {
            5 
        } else {
            0
        }
    };
    println!("var = {}", var);
}

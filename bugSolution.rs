fn main() {
    let mut x = 5;
    { // Use a block to limit the scope of the mutable borrow
        let y = &mut x;
        *y = 6;
    }
    { // Another block to limit the scope of the next mutable borrow
        let z = &mut x;
        *z = 7; 
    }
    println!("x = {}", x);
}
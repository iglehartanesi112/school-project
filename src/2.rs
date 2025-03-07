fn main() {
    let mut x = 0;
    let mut y = 0;
    
    while x < 10 && y < 10 {
        println!("x = {}", x);
        println!("y = {}", y);
        
        x += 1;
        y += 2;
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let data = String::from_utf8_lossy(&file.read_to_string("output.txt").unwrap());
    println!("{}", data);
}

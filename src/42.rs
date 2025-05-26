fn main() {
    // Example of handling an error
    match std::fs::read_to_string("example.txt") {
        Ok(data) => println!("File content: {}", data),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Example of converting a string to an integer
    let number = String::from("5");
    let parsed_number = number.parse::<i32>().unwrap();
    println!("Parsed number is: {}", parsed_number);

    // Example of using a loop with a while statement
    let mut count = 0;
    while true {
        println!("{}", count);
        count += 1;
        if count == 5 {
            break;
        }
    }

    // Example of using the range function to iterate over a sequence
    for i in 0..=5 {
        println!("Value: {}", i);
    }

    // Example of accessing an element in a vector with indexing
    let my_vector = vec![1, 2, 3];
    let index_to_access = 1;
    match &my_vector[index_to_access] {
        2 => println!("Element at index {} is 2", index_to_access),
        _ => eprintln!("Index out of bounds"),
    }

    // Example of using a switch-case statement with multiple cases
    let name = "Alice";
    if name == "Alice" {
        println!("Hello, Alice!");
    } else if name == "Bob" {
        println!("Hello, Bob!");
    } else {
        println!("Unknown user");
    }
}

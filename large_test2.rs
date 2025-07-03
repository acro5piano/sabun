use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();
    data.insert("name", "Alice");
    data.insert("age", "30");
    data.insert("city", "New York");
    
    for (field, info) in &data {
        println!("{}: {}", field, info);
    }
    
    let values = vec![10, 20, 30, 40, 50];
    let total: i32 = values.iter().sum();
    println!("Total: {}", total);
    
    if total > 100 {
        println!("Total is greater than 100");
    } else {
        println!("Total is 100 or less");
    }
    
    // Process each value with doubling
    for val in values {
        let doubled = val * 2;
        println!("{} doubled is {}", val, doubled);
    }
    
    let fib_result = compute_fibonacci(12);
    println!("Fibonacci(12) = {}", fib_result);
    
    // Additional computation
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}

fn compute_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => compute_fibonacci(n - 1) + compute_fibonacci(n - 2)
    }
}
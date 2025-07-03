use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    
    if sum > 10 {
        println!("Sum is greater than 10");
    } else {
        println!("Sum is 10 or less");
    }
    
    // Process each number
    for num in numbers {
        let squared = num * num;
        println!("{} squared is {}", num, squared);
    }
    
    let result = calculate_fibonacci(10);
    println!("Fibonacci(10) = {}", result);
}

fn calculate_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}
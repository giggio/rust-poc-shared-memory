use std::io::{self, Write};

mod shared_memory;
use shared_memory::*;

static LINK_FILE_NAME: &str = "/dev/shm/my_shmem_flink";

fn main() {
    env_logger::init();
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    println!("Done!");
}

fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    let arg = args.get(1).map(|s| s.as_str()).unwrap_or("u8");
    match arg {
        "array" => {
            println!("Using array...");
            use_array()?;
        }
        "struct" => {
            println!("Using struct...");
            use_struct()?;
        }
        "isinit" => {
            println!("Checking initialization...");
            is_initialized();
        }
        _ => {
            println!("Using u8...");
            use_u8()?;
        }
    }
    Ok(())
}

fn use_array() -> Result<(), String> {
    let shm = SharedMemory::<[u8;2]>::new(LINK_FILE_NAME)?;
    let current_value = shm.get();
    println!("Initial value in shared memory: {} {}", current_value[0], current_value[1]);
    loop {
        let mut input = String::new();
        print!("Enter a pair of integers separated by space: ");
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;  // Ensure the prompt is printed before reading input
        io::stdin().read_line(&mut input).map_err(|e| format!("Failed to read line: {}", e))?;
        let input = input.trim();
        if input == "" {
            println!("Exiting...");
            break;
        }
        println!("You entered: {}", input);
        let split_input = input.split_whitespace().collect::<Vec<&str>>();
        if split_input.len() != 2 {
            println!("Please enter exactly two integers.");
            continue;
        }
        let result_vec: Result<Vec<u8>, _> = split_input.iter().map(|&v| v.parse::<u8>()).into_iter().collect();
        let numbers = match result_vec {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid input, please enter two integers separated by an space.");
                continue;
            }
        };
        let value = [numbers[0], numbers[1]];
        let current_value = shm.get();
        println!("Previous value: {} {}", current_value[0], current_value[1]);
        shm.set(value)?;
    }
    Ok(())
}

#[derive(Default, Copy, Clone)]
struct MyStruct {
    a: char,
    b: char,
}

fn use_struct() -> Result<(), String> {
    let shm = SharedMemory::<MyStruct>::new(LINK_FILE_NAME)?;
    let current_value = shm.get();
    println!("Initial value in shared memory: {} {}", current_value.a, current_value.b);
    loop {
        let mut input = String::new();
        print!("Enter two characters: ");
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;  // Ensure the prompt is printed before reading input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "" {
            println!("Exiting...");
            break;
        }
        println!("You entered: {}", input);
        if input.len() != 2 {
            println!("Please enter exactly two characters.");
            continue;
        }
        let mut values = input.chars();
        let value = MyStruct {
            a: values.nth(0).unwrap(),
            b: values.nth(0).unwrap(),
        };
        let current_value = shm.get();
        println!("Previous value: {} {}", current_value.a, current_value.b);
        shm.set(value)?;
    }
    Ok(())
}

fn use_u8() -> Result<(), String> {
    let shm = SharedMemory::<u8>::new(LINK_FILE_NAME)?;
    let current_value = shm.get();
    println!("Initial value in shared memory: {}", current_value);
    loop {
        let mut input = String::new();
        print!("Enter an integer: ");
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;  // Ensure the prompt is printed before reading input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "" {
            println!("Exiting...");
            break;
        }
        println!("You entered: {}", input);
        let value: u8 = match input.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid input, please enter an integer.");
                continue;
            }
        };
        let current_value = shm.get();
        println!("Previous value: {}", current_value);
        shm.set(value)?;
    }
    Ok(())
}

fn is_initialized() -> bool {
    match SharedMemory::<u8>::is_initialized(LINK_FILE_NAME) {
        Ok(true) => {
            println!("Shared memory is initialized.");
            std::process::exit(0);
        }
        Ok(false) => {
            println!("Shared memory is not initialized.");
            std::process::exit(1);
        }
        Err(e) => {
            println!("Error checking shared memory initialization: {}", e);
            std::process::exit(1);
        }
    }
}


use std::env::args;
use transducer_byte_encryption::transducer::Transducer;

fn main() {
    // Get environment
    let args: Vec<String> = args().collect();
    // Encrypt/Decrypt path
    if args.len() == 4 {
        // Get Flag
        match args[1].chars().nth(1) {
            Some(c) => match c {
                // Decrypt
                'd' => {
                    // Load transducer
                    let t = match Transducer::load_transducer(&args[2]) {
                        Ok(t) =>  t,
                        Err(e) => return println!("{}", e.to_string()),
                    };
                    // Decrypt file
                    match t.decrypt_file(&args[3]) {
                        Ok(_) => {},
                        Err(e) => return println!("{}", e.to_string()),
                    }
                },
                // Encrypt
                'e' => {
                    // Load transducer
                    let t = match Transducer::load_transducer(&args[2]) {
                        Ok(t) => t,
                        Err(e) => return println!("{}", e.to_string()),
                    };
                    // Encrypt file
                    match t.encrypt_file(&args[3]) {
                        Ok(_) => {},
                        Err(e) => return println!("{}", e.to_string()),
                    }
                },
                // Incorrect flag
                _ => println!("Invalid arguments\nUse the -h flag to see commands"),
            },
            // Missing flag
            None => println!("Invalid arguments\nUse the -h flag to see commands"),
        }
    }
    // Generate path
    else if args.len() == 3 {
        match args[1].chars().nth(1) {
            Some(c) => match c {
                // Generate transducer
                'g' => {
                    // Construct random transducer
                    let t = Transducer::spawn();
                    // Save random transducer
                    match t.save_transducer(&args[2]) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e.to_string()),
                    };
                },
                // Incorrect flag
                _ => println!("Invalid arguments\nUse the -h flag to see commands"),
            },
            // Missing flag
            None => println!("Invalid arguments\nUse the -h flag to see commands"),
        }
    }
    // Help path
    else if args.len() == 2 {
        match args[1].chars().nth(1) {
            Some(c) => match c {
                // Help menu
                'h' => {
                    println!("\n-d <key path> <file path> Decrypts the file at path with key\n-e <key path> <file path> Encrypts the file at path with key\n-g <key path> Generates a key and stores it at path\n-h This menu\n",)
                },
                // Incorrect flag
                _ => println!("Invalid arguments\nUse the -h flag to see commands"),
            },
            // Missing flag
            None => println!("Invalid arguments\nUse the -h flag to see commands"),
        }
    }
    // Completely incorrect input
    else {
        println!("Invalid arguments\nUse the -h flag to see commands");
    }
}

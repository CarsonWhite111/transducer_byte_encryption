use std::{arch::x86_64::_mm_testz_pd, env::args};

use transducer_byte_encryption::transducer::Transducer;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 4 {
        match args[1].chars().nth(1) {
            Some(c) => match c {
                'd' => {
                    let t = match Transducer::load_transducer(&args[2]) {
                        Ok(t) =>  t,
                        Err(_) => todo!(),
                    };
                    match t.decrypt_file(&args[3]) {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    }
                },
                'e' => {
                    let t = match Transducer::load_transducer(&args[2]) {
                        Ok(t) => t,
                        Err(_) => todo!(),
                    };
                    match t.encrypt_file(&args[3]) {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    }
                },
                _ => todo!(),
            },
            None => todo!(),
        }
    }
    else if args.len() == 3 {
        match args[1].chars().nth(1) {
            Some(c) => match c {
                'g' => {
                    let t = Transducer::spawn();
                    match t.save_transducer(&args[2]) {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    };
                },
                _ => todo!(),
            },
            None => todo!(),
        }
    }
    else if args.len() == 2 {
        match args[1].chars().nth(1) {
            Some(c) => match c {
                'h' => todo!(),
                _ => todo!(),
            },
            None => todo!(),
        }
    }
    else {
        println!("Inalid argmunts\nUse the -h flag to see commands");
    }
}

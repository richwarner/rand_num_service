use std::{fs, thread, time::{Duration, SystemTime}, env::args};
use rand::prelude::*;

fn main() {

    let mut io_file: &str = "io.txt";

    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        io_file = &args[1];
    }
    
    loop {
        println!("Checking file for new request at {:?}", SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards"));

        match fs::read_to_string(io_file) {
            Ok(contents) => {

                // Check whether the file contains only a positive integer
                let is_digits = !contents.is_empty() && contents.bytes().all(|c| c.is_ascii_digit());

                if is_digits {
                    let num_rands = contents.parse::<i32>().unwrap();
                    println!("\nNew request found for {} random numbers", num_rands);

                    // Construct output
                    let mut new_contents = String::new();
                    for i in 0..num_rands {
                        let rand_num = 2.0 * random::<f32>();
                        if i > 0 {
                            new_contents.push('\n');
                        }
                        new_contents.push_str(&format!("{}", rand_num));
                    }
                    println!("Random numbers generated:\n{}\n", new_contents);

                    // Write output to file
                    fs::write(io_file, new_contents).expect("Unable to write file");
                } 
            },
            Err(e) => {
                println!("Error reading file: {}", e);
            }
        }

        // Sleep for half a second before checking again
        thread::sleep(Duration::from_millis(500));

    }
    
}

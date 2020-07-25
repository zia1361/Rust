use std::io;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("invalid input");
    let option:i32 = option.trim().parse().expect("failed to convert");
    if option == 1
    {
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).expect("invalid filename");
        let mut path = "D:/".to_string() + &filename;
        create(path.trim().to_string());
    }
    if option == 2
    {
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).expect("invalid input");
        let mut content = String::new();
        io::stdin().read_line(&mut content).expect("invalid input");
        let mut path = "D:/".to_string()+&filename;
        append(path.trim().to_string(), &content);

    }
    if option == 3
    {
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).expect("invalid input");
        let mut path = "D:/".to_string() + &filename;
        read(&path.trim().to_string());
    }
    
}

fn append(path_val: String, content: &String)
{
    let path = Path::new(&path_val);
    let mut file2 = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();
    writeln!(file2,"{}",&content);
    
}

fn read(path_val: &String){
    
    let path = Path::new(& path_val);
     let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}

fn create(path_val: String){
    let file = File::create(&path_val).expect("unable to create file");
}
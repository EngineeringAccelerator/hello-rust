use std::env;

mod accdb {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::{self, BufRead};
    use std::path::Path;

    const DB_PATH: &str = "acc.db";

    pub fn get(key: &str) -> String {
        if let Ok(lines) = read_lines(DB_PATH) {
            for line in lines {
                if let Ok(key_value) = line {
                    let vec: Vec<&str> = key_value.split(",").collect();
                    if vec[0] == key {
                        return vec[1].to_owned();
                    }
                }
            }
        }

        return String::new();
    }

    pub fn put(key: &str, value: &str) -> String {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(DB_PATH)
            .expect("failed open db file");

        if let Err(e) = writeln!(file, "{},{}", key, value) {
            println!("can not write to file {}", e);
        }

        return value.to_owned();
    }

    pub fn del(key: &str) -> String {
        println!("not implemented: {}", key);
        return String::new();
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (cmd, key, val) = parse_command(&args);

    // println!("CMD: {}", cmd);
    // println!("Params: {}", key);

    let result = match cmd {
        "get" => accdb::get(key),
        "put" => accdb::put(key, val),
        "del" => accdb::del(key),
        &_ => {
            // ""
            panic!("invalid command: {}", cmd)
        }
    };

    println!("OK: {}", result);
}

fn parse_command(args: &[String]) -> (&str, &str, &str) {
    let cmd = &args[1];
    let param1 = &args[2];
    let param2 = if &args.len() > &3 { &args[3] } else { "" };

    (cmd, param1, param2)
}

use std::{
    env, fs,
    io::{self, Read, Result},
};

fn get_content(file_name: Option<&str>) -> Result<String> {
    let mut input = String::new();
    if let Some(value) = file_name {
        fs::read_to_string(value)
    } else {
        let result = io::stdin().read_to_string(&mut input);
        if result.is_err() {
            Err(result.unwrap_err())
        } else {
            Ok(input)
        }
    }
}

fn get_output(result: String, file_name: Option<&str>) -> String {
    if let Some(value) = file_name {
        format!("{result} ${value}")
    } else {
        result
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag: Option<&str>;
    let file_name: Option<&str>;
    let args_length = args.len();
    if args_length == 1 {
        flag = None;
        file_name = None;
    } else if args_length == 2 && args[1].starts_with("-") {
        flag = Some(&args[1]);
        file_name = None;
    } else if args_length == 2 {
        flag = None;
        file_name = Some(&args[1]);
    } else {
        flag = Some(&args[1]);
        file_name = Some(&args[2]);
    }

    let contents = get_content(file_name).expect("Cannot read file");
    let result = match flag {
        Some("-c") => contents.bytes().count().to_string(),
        Some("-l") => contents.lines().count().to_string(),
        Some("-w") => contents.split_whitespace().count().to_string(),
        Some("-m") => contents.chars().count().to_string(),
        Some(_) => panic!("Not implemented option"),
        None => {
            let byte_count = contents.bytes().count();
            let line_count = contents.lines().count();
            let word_count = contents.split_whitespace().count();
            format!("{line_count}\t{word_count}\t{byte_count}")
        }
    };
    let output = get_output(result, file_name);
    println!("{}", output);
}

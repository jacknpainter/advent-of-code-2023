use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn sum(nums: Vec<u32>) -> u32 {
    let mut sum = 0;
    for n in &nums {
        sum += n
    }

    return sum;
}

fn parse_lines(lines: Vec<String>) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();

    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in lines {
        res.push(parse_line(re.clone(), line));
    }

    return res;
}

fn parse_line(re: Regex, line: String) -> u32 {
    let Some(caps) = re.captures(&line) else {
        panic!("no match")
    };

    let first_digit: String;
    match &caps[1] {
        "one" => first_digit = "1".to_string(),
        "two" => first_digit = "2".to_string(),
        "three" => first_digit = "3".to_string(),
        "four" => first_digit = "4".to_string(),
        "five" => first_digit = "5".to_string(),
        "six" => first_digit = "6".to_string(),
        "seven" => first_digit = "7".to_string(),
        "eight" => first_digit = "8".to_string(),
        "nine" => first_digit = "9".to_string(),
        _ => first_digit = caps[0].to_string()
    }

    let last_digit: String;
    match &caps[caps.len()-1] {
        "one" => last_digit = "1".to_string(),
        "two" => last_digit = "2".to_string(),
        "three" => last_digit = "3".to_string(),
        "four" => last_digit = "4".to_string(),
        "five" => last_digit = "5".to_string(),
        "six" => last_digit = "6".to_string(),
        "seven" => last_digit = "7".to_string(),
        "eight" => last_digit = "8".to_string(),
        "nine" => last_digit = "9".to_string(),
        _ => last_digit = caps[caps.len()-1].to_string(),
    }

    let num = format!("{first_digit}{last_digit}");

    print!("line: {}\ndigits: {} {}\nnum: {}\n\n", line, first_digit, last_digit, num);

    return num.parse::<u32>().unwrap();
}

fn main() {
    let lines = read_lines("./text.txt");

    let nums = parse_lines(lines);

    let res = sum(nums);

    println!("{}", res);
}

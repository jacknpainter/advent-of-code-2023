use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn parse_lines(lines: Vec<String>) -> HashMap<u32, HashMap<String, Vec<u32>>> {
    let mut res = HashMap::new();

    let re = Regex::new(r"Game (\d{1,3}): ([0-9A-Za-z;, ]+)").unwrap();

    for line in lines {
        let Some(caps) = re.captures(&line) else {
            panic!("no match")
        };

        let index = caps[1].parse::<u32>().unwrap();
        let mut map: HashMap<String, Vec<u32>> = HashMap::new();

        let cre = Regex::new(r"((\d{1,3}) (green|blue|red)(?:,|;)?)").unwrap();
        for (_, [_, amount, colour]) in cre.captures_iter(&caps[2]).map(|c| c.extract()) {         
            let val = map.get(colour);
            match val {
                Some(v) => {
                    map.get_mut(&colour.to_string()).unwrap().push(amount.parse::<u32>().unwrap());
                }
                None => {
                    map.insert(colour.to_string(), Vec::new());
                    map.get_mut(&colour.to_string()).unwrap().push(amount.parse::<u32>().unwrap());
                }
            }
        }

        res.insert(index, map);
    }

    return res;
}

fn calc_part_one(map: HashMap<u32, HashMap<String, Vec<u32>>>) {
    let mut index_vec: Vec<u32> = Vec::new();

    for (index, v) in map {
        let mut red = true;
        for val in v.get("red").unwrap() {
            if *val > 12 {
                red = false;
                break
            }

            red = true;
        }

        let mut green = true;
        for val in v.get("green").unwrap() {
            if *val > 13 {
                green = false;
                break;
            }
        }

        let mut blue = true;
        for val in v.get("blue").unwrap() {
            if *val > 14 {
                blue = false;
                break;
            }
        }

        if red && green && blue {
            index_vec.push(index);
        }
    }

    let mut sum = 0;
    for i in index_vec {
        sum += i;
    }

    println!("P1: {}", sum)
}

fn calc_part_two(map: HashMap<u32, HashMap<String, Vec<u32>>>) {
    let mut nums: Vec<u32> = Vec::new();

    for (_, v) in map {
        let mut max_red = 0;
        for val in v.get("red").unwrap() {
            if *val > max_red {
                max_red = *val;
            }
        }

        let mut max_green = 0;
        for val in v.get("green").unwrap() {
            if *val > max_green {
                max_green = *val;
            }
        }

        let mut max_blue = 0;
        for val in v.get("blue").unwrap() {
            if *val > max_blue {
                max_blue = *val;
            }
        }

        nums.push(max_red*max_green*max_blue);
    }

    let mut sum = 0;
    for i in nums {
        sum += i
    }

    println!("P2: {}", sum);
}

fn main() {
    let lines = read_lines("./input.txt");

    let map = parse_lines(lines);

    calc_part_one(map.clone());
    calc_part_two(map.clone());
}
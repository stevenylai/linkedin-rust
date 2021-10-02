use std::env;
use std::fs;
use std::collections;

fn main() {
    if env::args().len() < 2 {
        println!("Usage: {} <filename>", env::args().nth(0).unwrap());
        return;
    }
    let filename = env::args().nth(1).unwrap();
    let file_contents = fs::read_to_string(filename);
    match file_contents {
        Ok(v) => {
            let mut count_map = collections::HashMap::new();
            for r in v.split_whitespace() {
                let normalized = r.to_uppercase();
                let to_update = count_map.entry(normalized).or_insert(0);
                *to_update += 1;
            }
            let mut count_list: Vec<(String, i32)> = Vec::new();
            for (key, value) in count_map.iter() {
                count_list.push((String::from(key), *value));
            }
            count_list.sort_by(|v1, v2| (*v1).1.cmp(&(*v2).1));
            let mut displayed = 0;
            for i in 0..count_list.len() {
                println!("{} -> {}", count_list[i].0, count_list[i].1);
                displayed += 1;
                if displayed > 4 {
                    break;
                }
            }
        }
        Err(_) => {
            println!("Bad file")
        }
    }
}

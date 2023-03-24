use std::{fs};

fn main() {
    let file_path = "src/challenge_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let content = contents.lines().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>();

    for x in 100..content.len() {
        if check_sums(x, &content) { break; }
    }
}

fn check_sums(x: usize, array: &Vec<u128>) -> bool {
    let slice = &array[x-100..x];
    let v = &array[x];
    for n in 0..100  {
        for m in 0..100 {
            if n != m && *v == slice[n] + slice[m] {
                return false;
            }
        }
    }

    println!("crumbeled at {}, index {}", array[x], x);
    return true;
}
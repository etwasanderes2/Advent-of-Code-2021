// #![feature(iter_zip)]
// don't want to wait to download nightly
// 1460 to0 low
use std::fs;

fn main() {

    const WINDOW: i32 = 3;

    let contents = fs::read_to_string("input")
        .expect("No read");
    
    let mut lines = contents.lines()
        .map(|x| x.parse::<i32>().expect("NaN"));
    
    let mut lines_after = lines.clone();
    let mut window_sum: i32 = 0;
    for _ in 0..WINDOW {
        window_sum += lines.next().expect("Not enough numbers");
    }

    let mut result = 0;
    for now in lines {
        // hacky zip for iterators
        let after = lines_after.next().expect("second iterator broken");

        println!("win_sum: {}", window_sum);

        let last_window_sum = window_sum;
        window_sum += now - after;
        if window_sum > last_window_sum {
            result += 1;
        } 
    }

    println!("Result: {}", result);
}

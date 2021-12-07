use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("input")
        .expect("No read");
    
    let mut lines = contents.lines();
    let mut last: i32 = lines.next()
        .expect("No first line")
        .parse()
        .expect("First line not a number");
    let mut result = 0;
    for line in  lines {
        let current: i32 = line.parse()
            .expect("Not a number");
        if current > last {
            result += 1;
        }
        last = current;
    }

    println!("{}", result);
}

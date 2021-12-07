use std::fs;

fn parseline(line: &str, prefix: &str) -> Option<i32> {
    return if line.starts_with(prefix) {
        Some(
            line[prefix.len()+1..].parse::<i32>()
                .expect("NaN")
        )
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("input")
        .expect("No input file");

    let mut hpos = 0;
    let mut vpos = 0;
    for line in input.lines() {
        if let Some(distnace) = parseline(line, "forward") {
            hpos += distnace;
        }
        if let Some(distnace) = parseline(line, "down") {
            vpos += distnace;
        }
        if let Some(distance) = parseline(line, "up") {
            vpos -= distance;
        }
    }

    println!("hpos: {}\nvpos: {}\nproduct: {}", 
        hpos, vpos, hpos * vpos);
}

use std::fs;

fn bit_at(x: u64, pos: u8) -> u64 {
    return (x >> pos) & 1;
}

fn main() {
    let input = fs::read_to_string("input")
        .unwrap();

    let len = input.lines().next().unwrap().len();
    let mask: u64 = u64::MAX >> (64 - len);

    let lines = input.lines()
        .map(|x| u64::from_str_radix(x, 2).unwrap());

    let mut total: u64 = 0;
    let mut ones: Vec<u64> = vec![0; 64];

    for n in lines {
        total += 1;
        for p in 0..64 {
            ones[p] += bit_at(n, p as u8);
        }
    }

    let mut result: u64 = 0;
    for p in (0..64).rev() {
        result <<= 1;
        result |= (total / 2 < ones[p]) as u64;
    }

    let other = !result & mask;

    println!("vector: {:?}\ntotal: {}\nnormal: {}\nother: {}\nproduct: {}", 
        ones, total, result, other, result*other);
}

// 1324314 too high

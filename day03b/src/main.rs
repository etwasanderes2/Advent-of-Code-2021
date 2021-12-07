use std::fs;

fn bit_at(x: u64, pos: usize) -> u64 {
    return (x >> pos) & 1;
}

fn count(numbers: &Vec<u64>) -> (Vec<i32>, i32) {
    let mut total: i32 = 0;
    let mut ones: Vec<i32> = vec![0; 64];
    for n in numbers {
        total += 1;
        for p in 0..64 {
            ones[p] += bit_at(*n, p) as i32;
        }
    }
    return (ones, total);
}

fn main() {
    let input = fs::read_to_string("input")
        .unwrap();

    let len = input.lines().next().unwrap().len();
    // let mask: u64 = u64::MAX >> (64 - len);

    let lines: Vec<u64> = input.lines()
        .map(|x| u64::from_str_radix(x, 2).unwrap())
        .collect();


    // println!("vector: {:?}\ntotal: {}\nnormal: {}\nother: {}\nproduct: {}", 
    //     ones, total, result, other, result*other);
    
    // oxygen
    let mut oxl = lines.clone();
    for p in (0..len).rev() {
        println!("{:?}", oxl);
        let (count, total) = count(&oxl);
        println!("{}: {:?} / {}",
            p, count, total);
        let bit = (count[p] * 2 >= total) as u64;
        println!("bit: {}", bit);
        oxl.retain(|&x| bit_at(x, p) == bit);
        if oxl.len() == 1 {
            break;
        }
    }
    println!("oxygen: {}", oxl[0]);

    // CO2
    // also, lazy copying!
    let mut col = lines.clone();
    for p in (0..len).rev() {
        println!("{:?}", col);
        let (count, total) = count(&col);
        println!("{}: {:?} / {}",
            p, count, total);
        let bit = (count[p] * 2 < total) as u64;
        println!("bit: {}", bit);
        col.retain(|&x| bit_at(x, p) == bit);
        if col.len() == 1 {
            break;
        }
    }
    println!("co2: {}", col[0]);
    println!("product: {}", oxl[0] * col[0]);
}

// 1324314 too high

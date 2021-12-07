#![feature(generators)]
// #![feature(conservative_impl_trait)]

use std::fs;
use gen_iter::gen_iter;

fn check_card(card: &Vec<Vec<i32>>, crossed_off: &Vec<bool>) -> bool {
    // rows
    for row in card {
        if row.iter().all(|x| crossed_off[*x as usize]) {
            return true;
        }
    }
    // cols
    for col in 0..5 {
        let ok = gen_iter!({
            for row in 0..5 {
                yield card[row][col];
            }
        }).all(|x| crossed_off[x as usize]);
        if ok {
            return true;
        }
    }
    return false;
}

fn check_win(bingo_cards: &Vec<Option<Vec<Vec<i32>>>>, crossed_off: &Vec<bool>) -> Option<usize> {
    for (card_no, maybe_card) in bingo_cards.iter().enumerate() {
        if let Some(card) = maybe_card {
            if check_card(card, crossed_off) {
                return Some(card_no);
            }
        }
    }
    return None;
}

fn card_value(card: &Vec<Vec<i32>>, crossed_off: &Vec<bool>) -> i32 {
    return card
        .iter()
        .flatten()
        .filter(|x| !crossed_off[**x as usize])
        .sum();
}

fn main() {
    let input = fs::read_to_string("input")
        .unwrap();
    let mut lines = input.lines();

    let numbers = lines.next()
        .unwrap()
        .split(",")
        .map(
            |x| x.parse::<i32>().unwrap()
        );

    let mut bingo_cards: Vec<Option<Vec<Vec<i32>>>> =
        Vec::new();
    // let mut index: Vec<Vec<(usize, usize, usize)>> = 
    //     vec![Vec::new(); 100];
    let remaining_lines: Vec<&str> = lines.collect();
    
    for (card_no, txt_card) in remaining_lines.chunks(6).enumerate() {
        // println!("Chunk: {:?}", txt_card);
        let mut card: Vec<Vec<i32>> = Vec::with_capacity(5);
        for (row_no, txt_row) in txt_card.iter().skip(1).enumerate() {
            let mut row: Vec<i32> = Vec::with_capacity(5);
            for (col_no, txt_number) in txt_row.split_whitespace().enumerate() {
                // println!("{}", txt_number);
                let number = txt_number.parse().unwrap();
                row.push(number);
                // index[number as usize].push((card_no, row_no, col_no));
            }
            card.push(row);
        }
        bingo_cards.push(Some(card));
    }

    let mut crossed_off: Vec<bool> = vec![false; 100];

    for (i, num) in numbers.enumerate() {
        crossed_off[num as usize] = true;
        while let Some(winner) = check_win(&bingo_cards, &crossed_off) {
            println!("winner after adding {} in turn {} on board {}", num, i, winner);
            // card value
            let value: i32 = card_value(&bingo_cards[winner].as_ref().unwrap(), &crossed_off);
            bingo_cards[winner] = None;
            
            println!("Board value: {}, Answer: {}", value, value * num);
        }
    }

    
}



pub fn main() {
    let input = include_str!("../inputs/day01.txt");
    // let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    let pt1_answer = pt1_solve(input);
    println!("pt1 answer is {pt1_answer}");
    let pt2_answer = pt2_solve(input);
    println!("pt2 answer is {pt2_answer}");
}

fn neg_mod(n:i32, m:i32) -> i32 {
    let n = n % m;
    if n < 0 {
        return n + m;
    }
    n
}

fn pt1_solve(input: &str) -> i32 {
    // dial starts at 50
    // moves to the left or right where each line is D## where D \in {L,R}
    // password is every time the dial points at 0

    let (_, zeroes) = input
        .lines()
        .fold((50,0), turn_dial);

    zeroes
}

fn turn_dial((dial_pos, zero_count):(i32,i32), turn:&str) -> (i32,i32) {
    let dial_pos = neg_mod(dial_pos + translate_turn(turn), 100);

    match dial_pos {
        0 => (0, zero_count+1),
        _ => (dial_pos, zero_count)
    }
}

fn pt2_solve(input: &str) -> i32 {
    let (_, zeroes) = input
        .lines()
        .fold((50,0), spin_dial);

    zeroes
}

fn spin_dial((dial_pos,zero_count):(i32,i32), turn:&str) -> (i32,i32) {
    let new_dial_pos = dial_pos + translate_turn(turn);
    let crossed_zero = match 
        dial_pos*new_dial_pos < 0 // different signs
        || new_dial_pos == 0 // moved to 0
     {
        true => 1,
        false => 0,
    };
    let num_passes = (new_dial_pos/100).abs() + crossed_zero;
    // println!("passed {} times, dial_pos went from {} to {}",
    //     num_passes, dial_pos, new_dial_pos);


    (neg_mod(new_dial_pos, 100), zero_count + num_passes)
}

fn translate_turn(turn:&str) -> i32 {
    let sign = match &turn.chars().nth(0) {
        Some('L') => -1,
        Some('R') => 1,
        _ => panic!("move is not L or R")
    };
    let amt:i32 = turn[1..].parse::<i32>().expect("move should be a number");
    sign * amt
}

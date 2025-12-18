type Range = (u64,u64);

pub fn main() {
    let input = include_str!("../inputs/day02.txt");
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let pt1_answer = solve_pt1(input);
    println!("answer to pt1 is {pt1_answer}");


    let pt2_answer = solve_pt2(input);
    println!("answer to pt2 is {pt2_answer}");
}

fn solve_pt1(input:&str) -> u64 {
    get_id_ranges(input)
        .fold(0, |cnt:u64, range:Range| {
            cnt + sum_invalid_ids_pt1(range)
        })
}

fn solve_pt2(input:&str) -> u64 {
    get_id_ranges(input)
        .fold(0, |cnt:u64, range:Range| {
            cnt + sum_invalid_ids_pt2(range)
        })
}

fn get_id_ranges(input:&str) -> impl Iterator<Item=Range> {
    input
        .split(",")
        .map(|range_str| parse_range(range_str))
        .flatten()
}


fn parse_range(range_txt: &str) -> Option<Range> {
    let range_txt_split = range_txt.split('-').collect::<Vec<&str>>();
    let err = format!("{} is an invalid range", String::from(range_txt));
    if range_txt_split.len() != 2 {
        println!("{}", &err);
        return None
    }

    let err = format!("{} is an invalid range", String::from(range_txt));
    let min = range_txt_split[0]
        .trim()
        .parse::<u64>()
        .expect(&format!("{err} on '{}'", range_txt_split[0]));
    let max = range_txt_split[1]
        .trim()
        .parse::<u64>()
        .expect(&format!("{err} on '{}'", range_txt_split[1]));
    Some((min,max))
}

fn sum_invalid_ids_pt1(range: Range) -> u64 {
    let (min,max) = range;
    (min..max).fold(0, |cnt:u64, x:u64| {
        let num_digits = x.ilog10()+1;
        if num_digits % 2 == 0 {
            let halfway:u64 = 10_i64
                .pow(num_digits / 2)
                .try_into()
                .expect("10 to pow failed");
            let top = x / halfway;
            let bottom = x % halfway;
            if top == bottom {
                return cnt + x
            }
        }
        cnt
    })
}

fn sum_invalid_ids_pt2(range: Range) -> u64 {
    let (min,max) = range;
    (min..max+1).fold(0, |cnt:u64, x:u64| {
        if has_repeated_digits(x) { return cnt + x }
        cnt
    })
}

fn has_repeated_digits(n:u64) -> bool {
    let num_digits = n.ilog10()+1;
    for i in (1..num_digits).rev() {
        if num_digits % i != 0 {
            continue;
        }
        if split_int(n, i).windows(2).all(|w| w[0]==w[1]) {
            return true;
        }
    }
    false
}

fn split_int(n:u64, d:u32) -> Vec<u64> {
    let m = 10_u64.pow(d);
    let mut splits = Vec::new();
    let mut n = n;
    while n >= 1 {
        splits.push(n % m);
        n = n / m;
    }
    splits
}


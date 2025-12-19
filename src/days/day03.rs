pub fn main() {
        // let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let input = include_str!("../inputs/day03.txt").trim();

        let pt1_answer = pt1_solve(input);
        println!("pt1 answer is {pt1_answer}");
        let pt2_answer = pt2_solve(input);
        println!("pt2 answer is {pt2_answer}");
}

fn pt1_solve(input: &str) -> u32 {
        input
                .split('\n')
                .map(|xs| get_max_joltage_pt1(xs))
                .fold(0_u32, |sum, n| sum + n)
}

fn get_max_joltage_pt1(xs: &str) -> u32 {
        let digits = xs
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .fold([0,0], |best, curr| {
                        let best_num = 10*best[0] + best[1];
                        if best_num < 10*best[1] + curr {
                                return [best[1],curr]
                        } else if best_num < 10*best[0] + curr {
                                return [best[0], curr]
                        }
                        best
                });
        10*digits[0] + digits[1]
}


fn pt2_solve(input: &str) -> u64 {
        input
                .split('\n')
                .map(|xs| xs.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .map(|xll:Vec<u32>| get_max_joltage_pt2(xll))
                // .inspect(|n| println!("{}",n))
                .sum()
}

fn get_max_joltage_pt2(xl:Vec<u32>) -> u64 {
        let strlen = xl.len();
        let mut min_i = 0;
        // println!("{:?}", xl);
        [0_u32;12]
                .into_iter()
                .enumerate()
                .map(|(i,_)| {
                        // digit0:for elements 0 to len-12, find the first instance of max
                        // digitn+1:for elements digit[n] to len-(12-(n+1)), find first instance of max
                        let (max_i, max_x) = get_max(&xl, min_i, strlen-(12-i));
                        // println!("({min_i},?),({max_i},{max_x})");
                        min_i = max_i+1;
                        u64::from(max_x)
                })
                .fold(0_u64, |tot, x| 10_u64*tot + x)
}

fn get_max(xl:&Vec<u32>, min:usize, max:usize) -> (usize,u32) {
        assert!(min <= max);
        assert!(max < xl.len());
        xl
                .into_iter()
                .enumerate()
                .fold((0_usize,0_u32), |(bi,bx),(i,x)| {
                        if i >= min && i <= max && *x > bx {
                                return (i,*x)
                        }
                        (bi,bx)
                })
}

// fn get_min(xl:[(usize,u64);12]) -> (usize,(usize,u64)) {
//         let min = xl
//                 .iter()
//                 .enumerate()
//                 // .rev()
//                 .min_by(|(_,(_,ax)),(_,(_,bx))| ax.cmp(bx))
//                 .unwrap();
//     (min.0, *min.1)
// }

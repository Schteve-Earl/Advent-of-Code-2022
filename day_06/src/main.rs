use std::collections::HashSet;
fn run()
{
    let input = include_str!("../input.txt").chars().collect::<Vec<char>>();

    println!("Part 1 answer is {}", nth_marker_position(&input, 4));
    println!("Part 2 answer is {}", nth_marker_position(&input, 14));
}

fn nth_marker_position(s: &Vec<char>, n: usize) -> usize {
    let mut answer: usize = 0;
    for ele in s.windows(n).enumerate() {
        if !has_duplicate_chars(ele.1) {
            answer = ele.0 + n;
            break;
        }
    }
    return answer;
}

fn has_duplicate_chars(s: &[char]) -> bool {
    let mut set = HashSet::new();
    for c in s {
        if !set.insert(c) {
            return true;
        }
    }
    return false;
}

fn main() {
    run();
}

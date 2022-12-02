mod day_2 {
    use std::env;
    use std::fs;

    pub fn part_1() {
        let puzzle_input = get_puzzle_input();

        let mut points = 0;

        fs::read_to_string(puzzle_input)
            .unwrap().
            lines().
            map(|n| n.parse().unwrap())
            .collect::<Vec<String>>()
            .iter().
            for_each(|pair| {
                let vec = pair.split(" ")
                                            .map(|n| n.parse().unwrap())
                                            .collect::<Vec<char>>();  
                points += match_part_1(vec[0], vec[1]);
                });

        println!("Part one answer: {points}");

    }

    pub fn part_2() {
        let puzzle_input = get_puzzle_input();

        let mut points = 0;

        fs::read_to_string(puzzle_input)
            .unwrap().
            lines().
            map(|n| n.parse().unwrap())
            .collect::<Vec<String>>()
            .iter().
            for_each(|pair| {
                let vec = pair.split(" ")
                                            .map(|n| n.parse().unwrap())
                                            .collect::<Vec<char>>();  
                points += match_part_2(vec[0], vec[1]);
                });

        println!("Part two answer: {points}");

    }

    fn get_puzzle_input() -> String {
        let mut dir =  env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();
        dir.push_str("/input/input.txt");
        return dir;
    }

    fn match_part_1(e: char, p: char) -> i32
    {
        // X = Rock, Y = Paper, Z = Scissors
        // A = Rock, B = Paper, C = Scissors
        // Rock = 1, Paper = 2, Scissors = 3
        let mut score : i32 = 0;
        match p
        {
            'X' => {
                score += 1;
                match e{
                    'A' => score += 3,
                    'B' => score += 0,
                    'C' => score += 6,
                    _ => print!("Invalid")
            }},
            'Y' => {
                score += 2;
                match e{
                    'A' => score += 6,
                    'B' => score += 3,
                    'C' => score += 0,
                    _ => print!("Invalid")
                }},
            'Z' => {
                score += 3;
                match e{
                    'A' => score += 0,
                    'B' => score += 6,
                    'C' => score += 3,
                    _ => print!("Invalid")
                }},
            _ => println!("Invalid OUTER")
        }
        return score;
    }

    fn match_part_2(e: char, o: char) -> i32
    {
        // X = Lose, Y = Draw, Z = Win
        // Rock = 1, Paper = 2, Scissors = 3
        let mut score : i32 = 0;
        match o
        {
            'X' => {
                // No score increase because we lost
                match e{
                    'A' => score += 3,
                    'B' => score += 1,
                    'C' => score += 2,
                    _ => print!("Invalid")
            }},
            'Y' => {
                score += 3;
                match e{
                    'A' => score += 1,
                    'B' => score += 2,
                    'C' => score += 3,
                    _ => print!("Invalid")
                }},
            'Z' => {
                score += 6;
                match e{
                    'A' => score += 2,
                    'B' => score += 3,
                    'C' => score += 1,
                    _ => print!("Invalid")
                }},
            _ => println!("Invalid OUTER")
        }
        return score;
    }
}
fn main() {
    crate::day_2::part_1();
    crate::day_2::part_2();
}
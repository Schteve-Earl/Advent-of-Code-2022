mod day_04 {
    pub fn part_1()
    {
        let mut overlap_pairs = 0;
        
        include_str!("../input.txt")
            .to_string()
            .lines()
            .map(|n| n.to_string())
            .collect::<Vec<String>>().iter().for_each(|s| {
                let pairs = s.split(',').map(|n| n.to_string()).collect::<Vec<String>>();
                let first= pairs.first().unwrap().split('-').map(|i| i.parse().unwrap()).collect::<Vec<i32>>();
                let second= pairs.last().unwrap().split('-').map(|i| i.parse().unwrap()).collect::<Vec<i32>>();
                // I know there's a logical operator for all this, but it's too late for this shit
                if (first.first() <= second.first() && first.last() >= second.last()) || (second.first() <= first.first() && second.last() >= first.last()) {
                    overlap_pairs += 1;
                }
                
            });

        println!("Part 1 total is {overlap_pairs}");
    }

    pub fn part_2()
    {
        let mut overlap_pairs = 0;
        
        include_str!("../input.txt")
            .to_string()
            .lines()
            .map(|n| n.to_string())
            .collect::<Vec<String>>().iter().for_each(|s| {
                // Lord, forgive me for what I am about to do
                let pairs = s.split(',').map(|n| n.to_string()).collect::<Vec<String>>();
                let first= pairs.first().unwrap().split('-').map(|i| i.parse().unwrap()).collect::<Vec<i32>>();
                let second= pairs.last().unwrap().split('-').map(|i| i.parse().unwrap()).collect::<Vec<i32>>();
                if (first.first() <= second.first() && first.last() >= second.first()) ||
                   (second.first() <= first.first() && second.last() >= first.first()) {
                    overlap_pairs += 1;
                }
                
            });

        println!("Part 2 total is {overlap_pairs}");
    }

}

fn main() {
    crate::day_04::part_1();
    crate::day_04::part_2();
}

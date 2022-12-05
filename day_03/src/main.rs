mod day_03 {
    pub fn part_1()
    {
        let mut priority_total = 0;
        
        include_str!("../input.txt")
            .lines()
            .for_each(|s| {
                let (first, second) = s.split_at(s.len()/2);
                for c in first.chars() {
                    if second.find(c).is_some() {
                        priority_total += char_to_priority(c);
                        break;
                    }
                }
            });

        println!("Part 1 total is {priority_total}");
    }

    pub fn part_2()
    {
        let mut priority_total = 0;
        
        include_str!("../input.txt")
            .lines()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .chunks(3)
            .for_each(|s| {
                for c in s[0].chars() {
                    if s[1].find(c).is_some() && s[2].find(c).is_some() {
                        priority_total += char_to_priority(c);
                        break;
                    }
                }
            });

        println!("Part 2 total is {priority_total}");
    }


    fn char_to_priority(c: char) -> i32 {
        let ascii_code = c as i32;
        // 65-90 = A-Z
        if (65..=90).contains(&ascii_code) {
            return ascii_code - 65 + 27;
        }
        // 97-122 = a-z
        ascii_code - 97 + 1
    }

}

fn main() {
    crate::day_03::part_1();
    crate::day_03::part_2();
}

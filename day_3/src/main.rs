
#![allow(dead_code)]

mod day_3 {
    pub fn part_1()
    {
        let mut priority_total = 0;
        
        include_str!("../input.txt")
                            .to_string()
                            .lines()
                            .map(|n| n.to_string())
                            .collect::<Vec<String>>().iter().for_each(|s| {
                                let (first, second) = s.split_at(s.len()/2);
                                for c in first.chars() {
                                    if second.find(c) != None {
                                        priority_total += char_to_priority(c);
                                        break;
                                    }
                                }
                            });

        println!("Part 1 total is {priority_total}");
    }


    fn char_to_priority(c: char) -> i32 {
        let ascii_code = c as i32;
        // 65-90 = A-Z
        if ascii_code >= 65 && ascii_code <= 90 {
            return ascii_code - 65 + 27;
        }
        // 97-122 = a-z
        return ascii_code - 97 + 1;
    }

}

fn main() {
    crate::day_3::part_1();

}

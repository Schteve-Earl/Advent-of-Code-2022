
#![allow(dead_code)]

pub mod day_1 {
    use std::collections::HashMap;
    struct Elf {
        total: u32,
        calories: Vec<u32>,
    }

    impl Elf {
        fn sum_calories(&mut self) {
            self.total = self.calories.iter().sum();
        }

        fn add_calories(&mut self, calorie: u32) {
            self.calories.push(calorie);
        }

        fn get_calories(&mut self) -> u32 {
            self.total
        }
    }
    
    pub fn compute_1_calories()
    {
        let mut elves: HashMap<u32, Elf> = HashMap::new();
        let mut index: u32 = 0;
        
        let problem_input = include_str!("../input.txt")
                            .to_string()
                            .lines()
                            .map(|n| n.to_string())
                            .collect::<Vec<String>>();

        let mut most_calories: Vec<u32> = vec![0; 3];
        elves.insert(index, Elf { total: 0, calories: Vec::new() });

        for p in problem_input {
            if p.is_empty() {
                elves.entry(index).and_modify(| elf | { elf.sum_calories()});
                let mut calories: u32 = 0;
                elves.entry(index).and_modify(| elf | { calories = elf.get_calories()});

                for i in 0..most_calories.len() {
                    if most_calories[i] < calories {

                        most_calories.push(calories);
                        most_calories.sort();
                        most_calories.remove(0);
                        break;
                    }
                }
                index += 1;
                elves.insert(index, Elf { total: 0, calories: Vec::new() });
                continue;
            }
            elves.entry(index).and_modify(| elf | { elf.add_calories(p.parse().unwrap())});
        }

        let mc: u32 = most_calories.iter().sum();
        println!("{mc}");
    }

}

fn main() {
    crate::day_1::compute_1_calories();
}

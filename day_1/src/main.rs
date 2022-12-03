
#![allow(dead_code)]

mod day_1 {

    pub fn compute_calories(size: usize)
    {
        let mut r = 0;
        let mut vec: Vec<u32> = Vec::new();
        
        include_str!("../input.txt")
                            .to_string()
                            .lines()
                            .map(|n| n.to_string())
                            .collect::<Vec<String>>().iter().for_each(|c| 
                            { 
                                if !c.is_empty() {
                                    r += c.parse::<u32>().unwrap();
                                }
                                else {
                                    vec.push(r);
                                    r = 0;
                                }
                            });
        vec.sort();
        vec.reverse();
        vec.resize(size, 0);
        let result: u32 = vec.iter().sum();

        println!("Top {size} calories is {result}");
    }

}

fn main() {
    crate::day_1::compute_calories(1);
    crate::day_1::compute_calories(3);
}

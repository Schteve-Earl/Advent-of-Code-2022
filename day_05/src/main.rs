mod day_05 {
    use std::collections::HashMap;

    pub fn part_1()
    {
        let mut answer: String = String::new();
        let mut stack: HashMap<u8, Vec<char>> = HashMap::new();

        let input_split:Vec<&str> = include_str!("../input.txt").split("\n\n").collect();

        // Separate input into the puzzle header and the directions
        let pre_stack_ref = &input_split[0];
        let directions_ref = &input_split[1];

        // Determine starting positions
        pre_stack_ref.split('\n').for_each(|l| {
            for i in 0..l.len() {
                let c = l.chars().nth(i).unwrap();
                if c.is_alphabetic()
                {
                    let num: u8 = ((i as f32) / 4.0).floor() as u8 + 1;
                    // if the key hasn't been registered, do it now
                    if let std::collections::hash_map::Entry::Vacant(e) = stack.entry(num) {
                        let mut vec: Vec<char> = Vec::new();
                        vec.push(c);
                        e.insert(vec);
                    } else {
                        stack.entry(num).and_modify(|e| e.push(c));
                    }
                }
            }
        });

        // Reverse the stacks
        for i in 1..10 {
            stack.entry(i as u8).and_modify(|v| v.reverse());
        }

        // Trim the whitespace
        let directions = directions_ref.trim_end();
        directions.split('\n')
                    .for_each(|s| {
            // Create a vector that will hold all the directional commands
            let mut vec: Vec<u8> = Vec::new();
            let mut num: String = String::new();

            // Either I'm dumb or rust is clunky, so this is a manual mathod for splitting the strings into the three commands
            for i in 0..s.len() {
                let c = s.chars().nth(i).unwrap(); 
                if c.is_ascii_digit() {
                    num.push(c);
                }
                if !c.is_ascii_digit() && !num.is_empty() {
                    vec.push(num.parse::<u8>().unwrap());
                    num = String::new();
                }
        }
        // Push the final number
        vec.push(num.parse::<u8>().unwrap());                    

        // Finally, use the directional commands to move the boxes
        for i in 0..vec[0] {
            let mut c: char = 'a';
            stack.entry(vec[1]).and_modify(|v| c = v.pop().unwrap());
            stack.entry(vec[2]).and_modify(|v| v.push(c));
        }
    });

    for i in 1..10 {
        stack.entry(i as u8).and_modify(|v| answer.push(v.pop().unwrap()));
    }

    println!("Part 1 answer is {answer}");
        
    }

    pub fn part_2()
    {
        let mut answer: String = String::new();
        let mut stack: HashMap<u8, Vec<char>> = HashMap::new();

        let input_split:Vec<&str> = include_str!("../input.txt").split("\n\n").collect();

        // Separate input into the puzzle header and the directions
        let pre_stack_ref = &input_split[0];
        let directions_ref = &input_split[1];

        // Determine starting positions
        pre_stack_ref.split('\n').for_each(|l| {
            for i in 0..l.len() {
                let c = l.chars().nth(i).unwrap();
                if c.is_alphabetic()
                {
                    let num: u8 = ((i as f32) / 4.0).floor() as u8 + 1;
                    // if the key hasn't been registered, do it now
                    if let std::collections::hash_map::Entry::Vacant(e) = stack.entry(num) {
                        let mut vec: Vec<char> = Vec::new();
                        vec.push(c);
                        e.insert(vec);
                    } else {
                        stack.entry(num).and_modify(|e| e.push(c));
                    }
                }
            }
        });

        // Reverse the stacks
        for i in 1..10 {
            stack.entry(i as u8).and_modify(|v| v.reverse());
        }

        // Trim the whitespace
        let directions = directions_ref.trim_end();
        directions.split('\n')
                    .for_each(|s| {
            // Create a vector that will hold all the directional commands
            let mut vec: Vec<u8> = Vec::new();
            let mut num: String = String::new();

            // Either I'm dumb or rust is clunky, so this is a manual mathod for splitting the strings into the three commands
            for i in 0..s.len() {
                let c = s.chars().nth(i).unwrap(); 
                if c.is_ascii_digit() {
                    num.push(c);
                }
                if !c.is_ascii_digit() && !num.is_empty() {
                    vec.push(num.parse::<u8>().unwrap());
                    num = String::new();
                }
            }
            // Push the final number
            vec.push(num.parse::<u8>().unwrap());

            // Finally, use the directional commands to move the boxes
            let mut c: Vec<char> = Vec::new();
            stack.entry(vec[1]).and_modify(|v| {
                c = v[v.len()-(vec[0] as usize)..].to_vec();
                v.resize(v.len()-(vec[0] as usize), 'c')});

            stack.entry(vec[2]).and_modify(|v| {
                for i in 0..c.len() {
                    v.push(c[i])
                }
            });
        });

        for i in 1..10 {
            stack.entry(i as u8).and_modify(|v| answer.push(v.pop().unwrap()));
        }
        println!("Part 2 answer is {answer}");


}

}

fn main() {
    crate::day_05::part_1();
    crate::day_05::part_2();
}

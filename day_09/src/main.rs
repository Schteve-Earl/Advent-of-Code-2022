use std::collections::HashSet;
fn main() {
    let mut tail_squares: HashSet<(i64, i64)> = HashSet::new();

	// In form [x, y]
	let mut head: [i64; 2] = [0, 0];
	let mut tail: [i64; 2] = [0, 0];



	tail_squares.insert((0, 0));


    include_str!("../input.txt").lines().for_each(|l| {
        let command = l.split_whitespace().collect::<Vec<&str>>();
		// UP
		if command.first().unwrap().to_string() == 'U'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				head[1] += 1;
				moveTail(head, &mut tail);
				tail_squares.insert((tail[0], tail[1]));
			}
		}
		// Down
		if command.first().unwrap().to_string() == 'D'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				head[1] -= 1;
				moveTail(head, &mut tail);
				tail_squares.insert((tail[0], tail[1]));
			}
		}
		// Left
		if command.first().unwrap().to_string() == 'L'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				head[0] -= 1;
				moveTail(head, &mut tail);
				tail_squares.insert((tail[0], tail[1]));
			}
		}
		// Right
		if command.first().unwrap().to_string() == 'R'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				head[0] += 1;
				moveTail(head, &mut tail);
				tail_squares.insert((tail[0], tail[1]));

			}
		}

    });

    println!("Part 1 answer is {}", tail_squares.len());

	tail_squares.clear();


// Part two

// Now there are 10 knots, so I gotta do it 10 times for each iteration

	let mut rope: Vec<[i64; 2]> = Vec::new();
	for _i in 0..10 {
		rope.push([0,0]);
	}

	println!("ROPE {:?}", rope);

	include_str!("../input.txt").lines().for_each(|l| {
		// println!("---------------------------------------------");
		// println!("Head is {:?}, tail is {:?}", rope[0], rope[9]);
        let command = l.split_whitespace().collect::<Vec<&str>>();
		// UP
		if command.first().unwrap().to_string() == 'U'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				rope[0][1] += 1;
				for j in 0..rope.len()-1 {
					moveTail(rope[j], &mut rope[j+1]);
					tail_squares.insert((rope[9][0], rope[9][1]));	
				}
			}
		}
		// Down
		else if command.first().unwrap().to_string() == 'D'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				rope[0][1] -= 1;
				for j in 0..rope.len()-1 {
					moveTail(rope[j], &mut rope[j+1]);
					tail_squares.insert((rope[9][0], rope[9][1]));	
				}
			}
		}
		// Left
		else if command.first().unwrap().to_string() == 'L'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				rope[0][0] -= 1;
				for j in 0..rope.len()-1 {
					moveTail(rope[j], &mut rope[j+1]);
					tail_squares.insert((rope[9][0], rope[9][1]));	
				}
			}
		}
		// Right
		else if command.first().unwrap().to_string() == 'R'.to_string() {
			for i in 0..command.last().unwrap().parse::<u8>().unwrap() {
				// println!("R:        Head is {:?}, tail is {:?}", rope[0], rope[9]);
				// println!("ROPE BEFORE {:?}", rope[0]);
				rope[0][0] += 1;
				// println!("ROPE AFTER {:?}", rope[0]);

				for j in 0..rope.len()-1 {
					moveTail(rope[j], &mut rope[j+1]);
					tail_squares.insert((rope[9][0], rope[9][1]));	
				}

			}
		}

    });

    println!("Part 2 answer is {}", tail_squares.len());



}

fn moveTail(head: [i64; 2], tail: &mut [i64; 2])
{
					// More than two above
					if tail[0] == head[0] && tail[1] < head[1]-1 {
						tail[1] += 1;
						
					}
					// More than two below
					else if tail[0] == head[0] && tail[1] > head[1]+1 {
						tail[1] -= 1;
						
					}
					// More than two left
					else if tail[1] == head[1] && tail[0] > head[0]+1 {
						tail[0] -= 1;
						
					}
					// More than two right
					else if tail[1] == head[1] && tail[0] < head[0]-1 {
						tail[0] += 1;
						
					}
					// Not in same row or column
					else if tail[0] != head[0] && tail[1] != head[1] && 
					(i64::abs(head[0]-tail[0]) > 1 || i64::abs(head[1]-tail[1]) > 1) {
						if head[0] > tail[0] {
							tail[0] += 1;
						}
						else {
							tail[0] -= 1;
						}
						if head[1] > tail[1] {
							tail[1] += 1;
						}
						else {
							tail[1] -= 1;
						}
					}
}
fn main() {
    let mut trees: Vec<Vec<u8>> = Vec::new();

    include_str!("../input.txt").lines().for_each(|l| {
        trees.push(l.chars().map(|m| m.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
    });

	let mut answer = 0;

	for i in 0..trees.len() {
		for j in 0..trees[i].len() {
			let height = trees[i][j];
			let mut left = true;
			let mut right = true;
			let mut top = true;
			let mut bottom = true;
			for k in 0..i {
				if trees[k][j] >= height {
					top = false;
				}
			}
			for k in 0..j {
				if trees[i][k] >= height {
					left = false;
				}
			}
			for k in i+1..trees.len() {
				if trees[k][j] >= height {
					bottom = false;
				}
			}
			for k in j+1..trees[i].len() {
				if trees[i][k] >= height {
					right = false;
				}
			}
			if left || right || top || bottom; {
				answer += 1;
			}
		}
	}

    println!("Part 1 answer is {}", answer);

	answer = 0;


	for i in 0..trees.len() {
		for j in 0..trees[i].len() {
			let height = trees[i][j];
			let mut left = 0;
			let mut right = 0;
			let mut top = 0;
			let mut bottom = 0;
			for k in (0..i).rev() {
				top += 1;
				if trees[k][j] >= height {
					break;
				}
			}
			for k in (0..j).rev() {
				left += 1;
				if trees[i][k] >= height {
					break;
				}
			}
			for k in i+1..trees.len() {
				bottom += 1;
				if trees[k][j] >= height {
					break;
				}
			}
			for k in j+1..trees[i].len() {
				right += 1;
				if trees[i][k] >= height {
					break;
				}
			}
			let score = left * right * top * bottom;
			if score > answer {
				answer = score;
			}
		}
	}

    println!("Part 2 answer is {}", answer);
}
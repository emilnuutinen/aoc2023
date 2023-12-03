fn part1() -> color_eyre::Result<i32> {
    let mut score = 0;
    for line in include_str!("../../data/day02/input.txt").split('\n') {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
        } else {
            let game_number: i32 = parts[0]
                .trim()
                .trim_start_matches("Game")
                .trim()
                .parse()
                .ok()
                .unwrap_or(0);
            let games: Vec<&str> = parts[1].trim().split(';').collect();

            let mut possible = true;
            for game in games {
                let counts: Vec<&str> = game.trim().split(',').collect();
                for count in counts {
                    let count_parts: Vec<&str> = count.split_whitespace().collect();
                    let red_count = 12;
                    let green_count = 13;
                    let blue_count = 14;

                    let color = count_parts[1].to_lowercase();
                    let amount: i32 = count_parts[0].parse().unwrap();

                    match color.as_str() {
                        "red" => {
                            if amount > red_count {
                                possible = false;
                            }
                        }
                        "green" => {
                            if amount > green_count {
                                possible = false;
                            }
                        }
                        "blue" => {
                            if amount > blue_count {
                                possible = false;
                            }
                        }
                        _ => (),
                    }
                }
            }
            if possible {
                score += game_number;
            }
        }
    }
    Ok(score)
}

fn _part2() -> color_eyre::Result<()> {
    for _line in include_str!("../../data/day02/input.txt").split('\n') {}
    Ok(())
}

fn main() {
    let part1 = part1();
    //let part2 = part2();
    println!("Part 1 answer: {:?}", part1.unwrap());
    //println!("Part 2 answer: {:?}", part2.unwrap());
}

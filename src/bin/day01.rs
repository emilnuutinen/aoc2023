fn part1() -> color_eyre::Result<i32> {
    let mut sum = 0;
    for line in include_str!("../../data/day01/input.txt").split('\n') {
        let values: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        if !values.is_empty() {
            let num1 = values.first().unwrap();
            let num2 = values.last().unwrap();
            let combined_str = format!("{}{}", num1, num2);
            let combined_int: i32 = combined_str.parse().unwrap();
            sum += combined_int
        }
    }
    Ok(sum)
}

fn part2() -> color_eyre::Result<i32> {
    let mut sum = 0;
    for line in include_str!("../../data/day01/input.txt").split('\n') {
        let new_line = &line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        let values: Vec<u32> = new_line.chars().filter_map(|c| c.to_digit(10)).collect();
        if !values.is_empty() {
            let num1 = values.first().unwrap();
            let num2 = values.last().unwrap();
            let combined_str = format!("{}{}", num1, num2);
            let combined_int: i32 = combined_str.parse().unwrap();
            sum += combined_int
        }
    }
    Ok(sum)
}

fn main() {
    let part1 = part1();
    let part2 = part2();
    println!("Part 1 answer: {:?}", part1.unwrap());
    println!("Part 2 answer: {:?}", part2.unwrap());
}

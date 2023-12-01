fn main() -> color_eyre::Result<()> {
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
    println!("Part 1 answer: {sum}");

    Ok(())
}

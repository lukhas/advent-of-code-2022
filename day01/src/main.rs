use std::fs;

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("./input").unwrap();
    let elves: Vec<&str> = file.split("\n\n").collect();

    // sum each elf's calories
    let mut calories_by_elf: Vec<i32> = elves
        .iter()
        .map(|x| {
            x.split('\n')
                .fold(0, |sum, cal| sum + cal.parse::<i32>().unwrap())
        })
        .collect();

    calories_by_elf.sort();

    // the biggest single calories count is at the end of the sorted array
    println!("x: {}", calories_by_elf.last().unwrap());

    // ugly way to get the top 3
    println!(
        "y: {}",
        calories_by_elf
            .rchunks_exact(3)
            .next()
            .unwrap()
            .iter()
            .fold(0, |sum, cal| sum + cal)
    );

    Ok(())
}

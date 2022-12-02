use std::fs;

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("./input").unwrap();

    let elves: Vec<&str> = file.split("\n\n").collect();
    // println!("{} - {} - {}", f[0], f[8], f.last().unwrap());
    // println!("{}", f.len());

    let x = elves
        .iter()
        .map(|x| {
            x.split('\n')
                .fold(0, |sum, cal| sum + cal.parse::<i32>().unwrap())
        })
        .max()
        .unwrap();

    println!("x: {}", x);

    Ok(())
}

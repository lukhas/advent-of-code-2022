use std::collections::HashSet;
use std::fs;
use std::iter::zip;

fn main() -> std::io::Result<()> {
    let mut file = fs::read_to_string("./input").unwrap();
    if file.ends_with('\n') {
        file.pop();
    }
    let sacks: Vec<&str> = file.split('\n').collect();

    let x = sacks.iter().fold(0, |sum, sack| {
        //println!("sack: {}", sack);
        let (compart1, compart2) = sack.split_at(sack.len() / 2);
        //println!(" compartment1: {}\n compartment2: {}", compart1, compart2);

        let (mut h1, mut h2) = (HashSet::new(), HashSet::new());

        // list all the items
        for (c1, c2) in zip(compart1.chars(), compart2.chars()) {
            h1.insert(c1);
            h2.insert(c2);
        }

        let intersection: HashSet<_> = h1.intersection(&h2).collect();
        let c = *intersection.iter().next().unwrap();
        let ascii: u32 = *c as u32;
        //println!("{} - {}", c, ascii);
        let priority = if c.is_ascii_lowercase() {
            ascii - 96
        } else {
            ascii - 64 + 26
        };
        sum + priority
    });

    println!("x - {x}");

    Ok(())
}

fn main() {
    let input = include_str!("../../inputs/day1.txt");

    let mut password: u32 = 0;
    let mut dial: i32 = 50;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next();

        let mut amount: i32 = line
            .split_at(1)
            .1
            .parse()
            .expect("Failed to convert dial amount");

        match dir {
            Some('R') => {}
            Some('L') => {
                amount = -amount;
            }
            Some(c) => {
                println!("Invalid char: {}", c);
                continue;
            }
            None => {
                continue;
            }
        }

        dial += amount;
        if dial >= 100 {
            dial %= 100;
        } else if dial < 0 {
            dial = ((dial % 100) + 100) % 100;
        }

        if dial == 0 {
            password += 1;
        }
    }

    println!("Password: {}", password);
}

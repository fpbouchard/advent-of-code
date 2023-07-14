fn load_data() -> String {
    return std::fs::read_to_string("./data.txt").unwrap();
}

fn get_elves_calories() -> Vec<isize> {
    let contents = load_data();
    return contents.lines().fold(vec![0], |mut acc, line| {
        if line.len() > 0 {
            let last = acc.last_mut().unwrap();
            *last += line.parse::<isize>().unwrap();
        } else {
            acc.push(0)
        }
        acc
    });
}

fn part1() {
    let numbers = get_elves_calories();
    let largest = numbers.iter().max().unwrap();
    println!("{:?}", largest);
}

fn part2() {
    let mut numbers = get_elves_calories();
    numbers.sort_unstable();
    numbers.reverse();
    let iterator = numbers.iter();
    let top_three: isize = iterator.take(3).sum();
    println!("{:?}", top_three);
}

fn main() {
    part1();
    part2();
}

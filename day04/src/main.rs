fn next_letter(c: char) -> char {
    match c {
        'X' => 'M',
        'M' => 'A',
        _ => 'S',
    }
}

fn search_next(
    (x, y): (isize, isize),
    (dx, dy): (isize, isize),
    curr: char,
    table: &[Vec<char>],
) -> bool {
    let (nx, ny) = (x + dx, y + dy);
    if nx < 0 || ny < 0 {
        return false;
    }
    let ix = usize::try_from(nx).unwrap();
    let iy = usize::try_from(ny).unwrap();
    if ix >= table.len() || iy >= table[0].len() {
        return false;
    }
    let next = next_letter(curr);
    if table[ix][iy] != next {
        return false;
    }
    next == 'S' || search_next((nx, ny), (dx, dy), next, table)
}

fn search_xmas(table: &[Vec<char>]) -> u32 {
    let dirs = [
        (1, 0),
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
    ];
    let mut count = 0;
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            if table[i][j] == 'X' {
                for (dx, dy) in dirs.iter() {
                    count += search_next(
                        (i.try_into().unwrap(), j.try_into().unwrap()),
                        (*dx, *dy),
                        'X',
                        table,
                    ) as u32;
                }
            }
        }
    }
    count
}

fn count_crosses(table: &[Vec<char>]) -> usize {
    let mut count = 0;
    for i in 1..table.len() - 1 {
        for j in 1..table[0].len() - 1 {
            if table[i][j] != 'A' {
                continue;
            }
            let first = [table[i - 1][j - 1], table[i + 1][j + 1]]
                .iter()
                .collect::<String>();
            let second = [table[i - 1][j + 1], table[i + 1][j - 1]]
                .iter()
                .collect::<String>();
            if (first == "MS" || first == "SM") && (second == "MS" || second == "SM") {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("./input/04")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("Result: {}", search_xmas(&input));
    println!("Result: {}", count_crosses(&input));
}

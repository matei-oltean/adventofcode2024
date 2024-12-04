fn next_letter(c: char) -> char {
    match c {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => 'X',
    }
}

fn search_next(
    (x, y): (isize, isize),
    (dx, dy): (isize, isize),
    curr: char,
    table: &Vec<Vec<char>>,
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
    if table[ix][iy] != next_letter(curr) {
        return false;
    }
    if next == 'S' {
        return true;
    }
    search_next((nx, ny), (dx, dy), next, table)
}

fn search_xmas(table: &Vec<Vec<char>>) -> u32 {
    // generate all 8 possible directions.
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

fn main() {
    let input = std::fs::read_to_string("./input/04")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("Result: {}", search_xmas(&input));
}
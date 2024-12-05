use std::collections::HashMap;

fn get_middle_value(operations: &HashMap<usize, usize>) -> usize {
    *operations
        .iter()
        .find(|(_, v)| **v == (operations.len()) / 2)
        .unwrap()
        .0
}

fn is_valid(
    rules: &[(usize, usize)],
    operations: &HashMap<usize, usize>,
) -> Option<(usize, usize)> {
    for (a, b) in rules {
        if operations.contains_key(b)
            && operations.contains_key(a)
            && operations.get(a).unwrap() > operations.get(b).unwrap()
        {
            return Some((*a, *b));
        }
    }
    None
}

fn make_valid(rules: &[(usize, usize)], operations: &mut HashMap<usize, usize>) {
    while let Some((a, b)) = is_valid(rules, operations) {
        let temp = operations.insert(a, *operations.get(&b).unwrap()).unwrap();
        operations.insert(b, temp);
    }
}

fn main() {
    let mut rules: Vec<(usize, usize)> = Vec::new();
    let mut sum_valid = 0_usize;
    let mut sum_invalid = 0_usize;
    std::fs::read_to_string("./input/05")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                return;
            }
            if line.contains("|") {
                let mut parts = line.split("|");
                let a = parts.next().unwrap().parse::<usize>().unwrap();
                let b = parts.next().unwrap().parse::<usize>().unwrap();
                rules.push((a, b));
            } else {
                let mut operation: HashMap<usize, usize> = line
                    .split(",")
                    .enumerate()
                    .map(|(i, x)| (x.parse::<usize>().unwrap(), i))
                    .collect();
                if is_valid(&rules, &operation).is_none() {
                    sum_valid += get_middle_value(&operation);
                } else {
                    make_valid(&rules, &mut operation);
                    sum_invalid += get_middle_value(&operation);
                }
            }
        });
    println!("Result: {}", sum_valid);
    println!("Result: {}", sum_invalid);
}

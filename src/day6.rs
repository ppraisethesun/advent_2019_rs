#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let mut objects = line.split(")");
            (
                objects.next().unwrap().to_string(),
                objects.next().unwrap().to_string(),
            )
        })
        .collect()
}
fn count_orbits(map: &HashMap<&str, Vec<&str>>, parent: &str) -> usize {
    let count: usize;
    if let Some(children) = map.get(parent) {
        count = children
            .iter()
            .map(|child| count_orbits(&map, child))
            .fold(0, |total, c| total + c);
    } else {
        count = 0;
    }
    count + 1
}

use std::collections::*;
#[aoc(day6, part1)]
pub fn part1(orbits: &Vec<(String, String)>) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut objects = HashSet::new();

    for (parent, child) in orbits.iter() {
        objects.insert(child);
        map.entry(&parent)
            .and_modify(|children: &mut Vec<&str>| children.push(&child))
            .or_insert(vec![&child]);
    }

    objects
        .iter()
        .fold(HashMap::new(), |mut counts, parent| {
            if let None = counts.get(parent) {
                let more = count_orbits(&map, &parent);
                counts
                    .entry(parent)
                    .and_modify(|c| *c += more)
                    .or_insert(more);
            }
            counts
        })
        .iter()
        .fold(0, |total, (_, count)| total + count)
}

#[aoc(day6, part2)]
pub fn part2(orbits: &Vec<(String, String)>) -> usize {
    let mut child_parent_map: HashMap<&str, &str> = HashMap::new();

    for (parent, child) in orbits.iter() {
        child_parent_map.insert(&child, &parent);
    }

    let mut you = HashSet::new();
    let mut you_last = "YOU";
    while let Some(parent) = child_parent_map.get(you_last) {
        you.insert(parent);
        you_last = parent;
    }

    let mut san = HashSet::new();
    let mut san_last = "SAN";
    while let Some(parent) = child_parent_map.get(san_last) {
        san.insert(parent);
        san_last = parent;
    }

    you.difference(&san).count() + san.difference(&you).count()
}

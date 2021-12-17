use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{hash_map::Entry, HashMap};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut wires = Vec::new();
    let mut digits = Vec::new();
    for line in input.lines() {
        let mut parts = line.split("|");
        let wire: Vec<_> = parts
            .next()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.to_string())
            .collect();
        let digit: Vec<_> = parts
            .next()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.to_string())
            .collect();
        wires.push(wire);
        digits.push(digit);
    }
    (wires, digits)
}

#[aoc(day8, part1, d81)]
pub fn part1<'a>(input: &(Vec<Vec<String>>, Vec<Vec<String>>)) -> u64 {
    let (_, digits) = input;
    digits
        .iter()
        .flat_map(|n| n.iter())
        .filter(|n| match n.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count() as u64
}

const VALID: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

#[aoc(day8, part2, d82)]
pub fn part2<'a>(input: &(Vec<Vec<String>>, Vec<Vec<String>>)) -> u64 {
    let (wires, output) = input;
    let valid_mapping = to_frequency(VALID.iter().flat_map(|n| n.chars()));
    let mut lookup = HashMap::new();
    VALID.iter().enumerate().for_each(|(i, s)| {
        lookup.insert(*s, char::from_digit(i as u32, 10).unwrap());
    });

    let mut result = 0;
    for i in 0..wires.len() {
        let (wire, output) = (&wires[i], &output[i]);
        let freq = to_frequency(wire.iter().flat_map(|n| n.chars()));
        let mapping = resolve_mapping(valid_mapping, freq, wire);
        let mut display = String::new();
        for num in output.iter() {
            let mut chars: Vec<_> = num.chars().map(|n| *mapping.get(&n).unwrap()).collect();
            chars.sort();
            let signal: String = chars.into_iter().collect();
            let ch = *lookup.get(signal.as_str()).unwrap();
            display.push(ch);
        }
        result += display.parse::<u64>().unwrap();
    }
    result
}

fn to_frequency<I: IntoIterator<Item = char>>(iter: I) -> [u8; 7] {
    let mut occurences = [0; 7];
    for c in iter {
        let idx = (c as u8 - 'a' as u8) as usize;
        occurences[idx] += 1;
    }
    occurences
}

fn resolve_mapping(valid: [u8; 7], mapping: [u8; 7], parts: &[String]) -> HashMap<char, char> {
    let mut valid_map = HashMap::new();
    // Get unique occurence mappings
    for (i, count) in valid.into_iter().enumerate() {
        match valid_map.entry(count) {
            Entry::Occupied(o) => {
                o.remove_entry();
            }
            Entry::Vacant(e) => {
                e.insert((i as u8 + 'a' as u8) as char);
            }
        }
    }
    let mut out = HashMap::new();
    for (i, count) in mapping.iter().enumerate() {
        if let Some(&c) = valid_map.get(count) {
            let cur_char = (i as u8 + 'a' as u8) as char;
            out.insert(cur_char, c);
        }
    }

    // Find a and c
    let one = parts.iter().find(|n| n.len() == 2).unwrap();
    let seven = parts.iter().find(|n| n.len() == 3).unwrap();
    for sc in seven.chars() {
        let mut exists = false;
        for oc in one.chars() {
            if sc == oc {
                exists = true;
            }
        }
        if !exists {
            out.insert(sc, 'a');
            let c = mapping
                .iter()
                .enumerate()
                .find_map(|(i, count)| {
                    let cc = (i as u8 + 'a' as u8) as char;
                    if *count == 8 && cc != sc {
                        Some(cc)
                    } else {
                        None
                    }
                })
                .unwrap();
            out.insert(c, 'c');
        }
    }

    // Find d and g
    let four = parts.iter().find(|n| n.len() == 4).unwrap();
    let d = four.chars().find(|c| !out.contains_key(&c)).unwrap();
    out.insert(d, 'd');
    let g = mapping
        .iter()
        .enumerate()
        .find_map(|(i, count)| {
            let cc = (i as u8 + 'a' as u8) as char;
            if *count == 7 && cc != d {
                Some(cc)
            } else {
                None
            }
        })
        .unwrap();
    out.insert(g, 'g');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 61229);
    }
}

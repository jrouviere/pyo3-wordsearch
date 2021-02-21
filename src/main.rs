use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    let cyls = [
        ['x', 'u', 's', 'a', 'n', 'e', 'i', 'w', 'y', 'o'],
        ['h', 'e', 'b', 'i', 'd', 'u', 't', 'a', 'c', 'o'],
        ['l', 'i', 'n', 'a', 'z', 'u', 'f', 'o', 'm', 'e'],
        ['s', 'u', 'h', 'o', 'd', 'e', 'p', 'a', 'r', 'i'],
        ['f', 'a', 'k', 'n', 'e', 'r', 'i', 'l', 'o', 'b'],
        ['e', 'j', 'v', 'a', 'w', 'd', 'i', 'q', 't', 'r'],
        ['s', 'g', 'p', 'e', 'w', 't', 'n', 'v', 'a', 'l'],
        ['y', 'o', 'r', 'c', 'u', 'g', 'm', 't', 'n', 'e'],
    ];
    let content = fs::read_to_string("./8letter_englishwords.dat").expect("cannot read dictionary");
    let words: Vec<&str> = content.split_whitespace().collect();
    
    let all_results = search_all(cyls, words);

    for res in all_results {
        println!("{:?}", res)
    }
}

fn search_all(cyls: [[char; 10]; 8], words: Vec<&str>) -> Vec<(&str, Vec<usize>)> {
    let char_to_cyl = build_char_to_cyl(cyls);

    let empty: Vec<usize> = Vec::new();

    return words
        .par_iter()
        .map(|word| {
            let mut opts: [&Vec<usize>; 8] = [&empty; 8];
            for (idx, ch) in word.chars().enumerate() {
                opts[idx] = &char_to_cyl[&ch];
            }
            return (word, search(opts));
        })
        .filter(|(_word, res)| res.is_some())
        .map(|(word, res)| (*word, res.unwrap()))
        .collect();
}

fn search(options: [&Vec<usize>; 8]) -> Option<Vec<usize>> {
    let mut taken: [bool; 8] = [false; 8];
    let mut current: Vec<usize> = Vec::with_capacity(8);

    let mut i = 0;
    loop {
        let idx = current.len();
        // try
        loop {
            if i >= options[idx].len() {
                // backtrack
                if idx == 0 {
                    return None;
                }

                i = current.pop().unwrap();
                let wheel = options[idx - 1][i];
                taken[wheel] = false;
                i += 1;
                break;
            }
            let wheel = options[idx][i];
            if !taken[wheel] {
                // advance
                current.push(i);
                if current.len() == 8 {
                    return Some(
                        current
                            .iter()
                            .zip(options.iter())
                            .map(|(i, opt)| opt[*i] + 1)
                            .collect(),
                    );
                }
                taken[wheel] = true;
                i = 0;
                break;
            } else {
                // skip
                i += 1;
            }
        }
    }
}

fn build_char_to_cyl(cyls: [[char; 10]; 8]) -> HashMap<char, Vec<usize>> {
    let mut char_to_cyl: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, cy) in cyls.iter().enumerate() {
        for ch in cy.iter() {
            let v = char_to_cyl.entry(*ch).or_insert(Vec::new());
            v.push(i);
        }
    }
    return char_to_cyl;
}

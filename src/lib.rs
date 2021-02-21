use rayon::prelude::*;
use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

const WLEN: usize = 8;
const WCOUNT: usize = 10;

fn search_all_impl(cyls: [[char; WCOUNT]; WLEN], words: Vec<&str>) -> Vec<(&str, Vec<usize>)> {
    let char_to_cyl = build_char_to_cyl(cyls);

    let empty: Vec<usize> = Vec::new();

    return words
        .par_iter()
        .map(|word| {
            // can probably speed that up a bit
            let mut opts: [&Vec<usize>; WLEN] = [&empty; WLEN];
            for (idx, ch) in word.chars().enumerate() {
                opts[idx] = &char_to_cyl[&ch];
            }
            return (word, search(opts));
        })
        .filter(|(_word, res)| res.is_some())
        .map(|(word, res)| (*word, res.unwrap()))
        .collect();
}

fn search(options: [&Vec<usize>; WLEN]) -> Option<Vec<usize>> {
    let mut taken: [bool; WLEN] = [false; WLEN];
    let mut current: Vec<usize> = Vec::with_capacity(WLEN);

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
                if current.len() == WLEN {
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

fn build_char_to_cyl(cyls: [[char; WCOUNT]; WLEN]) -> HashMap<char, Vec<usize>> {
    let mut char_to_cyl: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, cy) in cyls.iter().enumerate() {
        for ch in cy.iter() {
            let v = char_to_cyl.entry(*ch).or_insert(Vec::new());
            v.push(i);
        }
    }
    return char_to_cyl;
}

#[pyfunction]
fn search_all(cyls: [[char; WCOUNT]; WLEN], words: Vec<&str>) -> PyResult<Vec<(&str, Vec<usize>)>> {
    Ok(search_all_impl(cyls, words))
}

#[pymodule]
fn wordsearch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(search_all, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::search_all_impl;
    use std::fs;

    #[test]
    fn it_works() {
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
        let content =
            fs::read_to_string("./8letter_englishwords.dat").expect("cannot read dictionary");
        let words: Vec<&str> = content.split_whitespace().collect();
        let all_results = search_all_impl(cyls, words);

        assert_eq!(all_results.len(), 10077)
    }
}

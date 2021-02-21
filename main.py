#!/usr/bin/env python3

import wordsearch
import timeit

if __name__ == '__main__':
    cyls = [
        ["x", "u", "s", "a", "n", "e", "i", "w", "y", "o"],
        ["h", "e", "b", "i", "d", "u", "t", "a", "c", "o"],
        ["l", "i", "n", "a", "z", "u", "f", "o", "m", "e"],
        ["s", "u", "h", "o", "d", "e", "p", "a", "r", "i"],
        ["f", "a", "k", "n", "e", "r", "i", "l", "o", "b"],
        ["e", "j", "v", "a", "w", "d", "i", "q", "t", "r"],
        ["s", "g", "p", "e", "w", "t", "n", "v", "a", "l"],
        ["y", "o", "r", "c", "u", "g", "m", "t", "n", "e"],
    ]

    with open("./8letter_englishwords.dat") as f:
        words = [line.strip().lower() for line in f.readlines()]
    words = list(words)


    def run():
        res = wordsearch.search_all(cyls, words)
        assert len(res) == 10077

    print(f"time: {timeit.timeit(run, number=1)}")

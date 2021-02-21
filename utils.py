"""
Helper functions for evaluating solutions.

"""


__all__ = (
    'check_solver',
)


import random
import string


cys = [
    ["x", "u", "s", "a", "n", "e", "i", "w", "y", "o"],
    ["h", "e", "b", "i", "d", "u", "t", "a", "c", "o"],
    ["l", "i", "n", "a", "z", "u", "f", "o", "m", "e"],
    ["s", "u", "h", "o", "d", "e", "p", "a", "r", "i"],
    ["f", "a", "k", "n", "e", "r", "i", "l", "o", "b"],
    ["e", "j", "v", "a", "w", "d", "i", "q", "t", "r"],
    ["s", "g", "p", "e", "w", "t", "n", "v", "a", "l"],
    ["y", "o", "r", "c", "u", "g", "m", "t", "n", "e"],
]


def _solve_word_simple_rbts_guts(word, cys, cy_nums):
    if not word:
        return []

    c = word[0]
    for idx, (cy, cy_num) in enumerate(zip(cys, cy_nums)):
        if c in cy:
            cys_next = cys[:]
            cy_nums_next = cy_nums[:]
            del cys_next[idx]
            del cy_nums_next[idx]
            solution = _solve_word_simple_rbts_guts(word[1:], cys_next, cy_nums_next)
            if solution is not None:
                return [cy_num] + solution
    return None


def _solve_word_simple_rbts(word):
    return _solve_word_simple_rbts_guts(word, cys, list(range(1, len(cys) + 1)))


def _solve_words_reference(words):
    """Simple recursive back-tracking search solution, for comparison."""
    for word in words:
        sol = _solve_word_simple_rbts(word)
        if sol is not None:
            yield (word, sol)


def _check_solution(word, sol):
    if len(sol) != len(word):
        raise Exception("Not enough wheels in solution")
    if len(set(sol)) != len(sol):
        raise Exception("Wheel used twice")
    for c, cy_num in zip(word, sol):
        if c not in cys[cy_num - 1]:
            raise Exception("Letter {} not on wheel {}".format(c, cy_num - 1))


def load_words():
    """Load a list of 8 letter words (that consist of the standard, unaccented, English alphabet only)."""
    with open("./8letter_englishwords.dat") as f:
        words = [line.strip().lower() for line in f.readlines()]
    words = list(words)
    random.shuffle(words)
    return words


def check_solver(solver_fn):
    """For each word with a solution check the solution works, and check the unsolvable words match the reference."""
    words = load_words()
    unsolvable = set(words)
    for word, sol in solver_fn(cys):
        _check_solution(word, sol)
        unsolvable.remove(word)

    unsolvable_ref = set(words)
    for word, sol in _solve_words_reference(words):
        unsolvable_ref.remove(word)

    if unsolvable != unsolvable_ref:
        raise Exception("Different set of unsolvable", len(unsolvable), len(unsolvable_ref))

    print("all good")
from functools import reduce


# Type-Aliases
Set = frozenset[str]
Config = list[Set]
Entry = tuple[Config, Config]

# Mapping from segments to values
VALUES: dict[Set, int] = {
    frozenset('abcefg'): 0,
    frozenset('cf'): 1,
    frozenset('acdeg'): 2,
    frozenset('acdfg'): 3,
    frozenset('bcdf'): 4,
    frozenset('abdfg'): 5,
    frozenset('abdefg'): 6,
    frozenset('acf'): 7,
    frozenset('abcdefg'): 8,
    frozenset('abcdfg'): 9,
}


def decode(mapping: dict[str, str], digit: Set) -> int:
    """Return the int value of `digit` encoded via the `mapping` dict"""
    return VALUES[frozenset(map(mapping.get, digit))]


def solve(entry: Entry) -> int:
    def len_filter(length: int, iterable, collect=next):
        """
        Filter elements of `iterable` of length `length` 
        and collect them using the collect callable
        """
        return collect(filter(lambda c: len(c) == length, iterable))

    config, code = entry
    # isolating each segment by combining the information on each digit
    _069: Config = len_filter(6, config, collect=list)
    _235: Config = len_filter(5, config, collect=list)
    _1: Set = len_filter(2, config)
    _4: Set = len_filter(4, config)
    _7: Set = len_filter(3, config)
    _8: Set = len_filter(7, config)
    g = len_filter(1, map(lambda num: num - (_4 | _7), _069))
    d = len_filter(1, map(lambda num: num - (_7 | g), _235))
    a = _7 - _1
    b = _4 - (_1 | d)
    _5: Set = next(filter(lambda num: b <= num, _235))
    f = _5 - a.union(b, d, g)
    c = _1 - f
    e = _8 - a.union(b, c, d, f, g) # the remaining one
    # mapping from coded to decoded letter
    mapping = dict(zip(
        map(lambda s: next(iter(s)), [a, b, c, d, e, f, g]), 
        'abcdefg'
    ))
    return reduce(lambda acc, digit: 10*acc + decode(mapping, digit), code, 0)


def solution01(entries: list[Entry]) -> int:
    return sum(map(
        lambda entry: len([code for code in entry[-1] if len(code) in [2, 3, 4, 7]]),
        entries
    ))


def solution02(entries: list[Entry]):
    return sum(map(solve, entries))


def main():
    with open("input.txt") as file:
        entries = [
            tuple(map(lambda f: [frozenset(c) for c in f.split()], line.strip().split(" | ")))
            for line in file
        ]
    print(solution01(entries))
    print(solution02(entries))


if __name__ == "__main__":
    main()

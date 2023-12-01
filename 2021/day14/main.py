from collections import Counter


def pairs(it):
    return zip(it[:-1], it[1:])


def polymerize1(seed: str, rules: dict[str, str]) -> str:
    return "".join(
        f"{a}{rules[a+b]}" for a, b in pairs(seed)
    ) + seed[-1]


def polymerize2(seed: Counter, count: Counter, rules: dict[str, str]) -> Counter:
    polymer = Counter()
    newcount = count.copy()
    for (a, b), num in seed.items():
        i = rules[a+b]
        polymer[a+i] += num
        polymer[i+b] += num
        newcount[i] += num
    return polymer, newcount


def solution1(seed, rules):
    polymer = seed
    for _ in range(10):
        polymer = polymerize1(polymer, rules)
    count = Counter(polymer)
    most_common = count.most_common()
    first, last = most_common[0], most_common[-1]
    return first[-1] - last[-1]


def solution2(seed, rules):
    polymer = Counter("".join(win) for win in pairs(seed))
    count = Counter(seed)
    for _ in range(40):
        polymer, count = polymerize2(polymer, count, rules)
    most_common = count.most_common()
    first, last = most_common[0], most_common[-1]
    return first[-1] - last[-1]


def main():
    with open("input.txt") as file:
        it = map(str.strip, file)
        seed = next(it)
        next(it)
        rules = { pat: i for pat, i in map(lambda r: r.split(" -> "), it) }
    print(solution1(seed, rules))
    print(solution2(seed, rules))


if __name__ == "__main__":
    main()

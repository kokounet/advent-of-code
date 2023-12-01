from pathlib import Path

ALPHABET = "abcdefghijklmnopqrstuvwxyz"
PRIORITY: dict[str, int] = {
    c: (i + 1) for (i, c) in enumerate(ALPHABET + ALPHABET.upper())
}


def part1(content: list[str]) -> int:
    score = 0
    for sack in content:
        mid = int(len(sack) / 2)
        c1, c2 = set(sack[:mid]), set(sack[mid:])
        for c in c1 & c2:
            score += PRIORITY[c]
    return score


def part2(content: list[str]) -> int:
    score = 0
    for i in range(int(len(content) / 3)):
        group = iter([set(sack) for sack in content[3 * i : 3 * (i + 1)]])
        badge = next(group)
        for sack in group:
            badge &= sack
        for c in badge:
            score += PRIORITY[c]
    return score


def main():
    with Path("day03/input.txt").open() as file:
        content = [line.strip() for line in file]
    print(part1(content))
    print(part2(content))


if __name__ == "__main__":
    main()

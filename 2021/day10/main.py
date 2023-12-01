from functools import reduce


MATCHING = {'(': ')', '[': ']', '<': '>', '{': '}'}
SCORES1 = {')': 3, ']': 57, '}': 1197, '>': 25137}
SCORES2 = {')': 1, ']': 2, '}': 3, '>': 4}


def parse1(line):
    stack = []
    for token in line:
        if token in MATCHING:
            stack.append(MATCHING[token])
        else:
            if not stack:
                return 0
            expected = stack.pop()
            if token != expected:
                return SCORES1[token]
    return 0


def parse2(line):
    stack = []
    for token in line:
        if token in MATCHING:
            stack.append(MATCHING[token])
        else:
            if not stack:
                return None
            expected = stack.pop()
            if token != expected:
                return None
    return reduce(lambda acc, tok: 5*acc+SCORES2[tok], reversed(stack), 0)
            

def solution1(lines):
    return sum(map(parse1, lines))


def solution2(lines):
    scores = sorted(filter(lambda score: score is not None, map(parse2, lines)))
    return scores[len(scores) // 2]

def main():
    with open("example.txt") as file:
        lines = [list(line.strip()) for line in file]
    print(solution1(lines))
    print(solution2(lines))


if __name__ == "__main__":
    main()

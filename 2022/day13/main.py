from pathlib import Path
from itertools import zip_longest
from functools import cmp_to_key


def main():
    with Path("day13/input.txt").open() as file:
        packets = [eval(line.strip()) for line in file if line.strip()]
    pairs = list(zip(packets[::2], packets[1::2]))
    print(part1(pairs))
    print(part2(packets))


def part1(pairs):
    count = 0
    for i, (left, right) in enumerate(pairs):
        if order(left, right) < 0:
            count += i + 1
    return count


def part2(packets):
    packets = list(packets)
    packets.extend([[[2]], [[6]]])  # add divider packets
    packets.sort(key=cmp_to_key(order))
    return (packets.index([[2]]) + 1) * (packets.index([[6]]) + 1)


def order(left, right) -> int:
    """
    returns:
        - a negative number if left is lower than right
        - 0 if left is equal to right
        - a positive number if left is greater than right
    """
    if isinstance(left, int) and isinstance(right, int):
        return left - right
    if isinstance(left, int):
        left = [left]
    if isinstance(right, int):
        right = [right]
    it = zip_longest(left, right)
    cmp = 0
    for (left, right) in it:
        if left is None:
            return -1
        if right is None:
            return 1
        cmp = order(left, right)
        if cmp == 0:
            continue
        return cmp
    return cmp


if __name__ == "__main__":
    main()

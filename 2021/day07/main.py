import numpy as np


def solution1(positions: np.ndarray) -> int:
    """
    MATHS!!! YEAH, the median is the value that minimizes the mean of the absolute
    differences, therefore it also minimizes the sum of the absolute differences
    """
    return int(np.sum(np.abs(positions - np.median(positions))))


def solution2(positions: np.ndarray) -> int:
    values = np.arange(positions.min(), positions.max()+1)
    matrix = np.abs(np.tile(positions, (values.size, 1)).T - values)
    matrix = matrix * (matrix + 1) / 2 # sum from 0 to n for each element
    return int(matrix.sum(0).min())


def main():
    positions = np.loadtxt("input.txt", dtype=int, delimiter=',')
    print(solution1(positions))
    print(solution2(positions))


if __name__ == "__main__":
    main()
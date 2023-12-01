import numpy as np
from scipy.ndimage import label


def solution1(terrain: np.ndarray) -> int:
    lowpoint = np.ones_like(terrain, dtype=bool)
    # up neighbor
    lowpoint[:-1, :] &= terrain[:-1, :] < terrain[1:, :]
    # down neighbor
    lowpoint[1:, :] &= terrain[1:, :] < terrain[:-1, :]
    # right neighbor
    lowpoint[:, :-1] &= terrain[:, :-1] < terrain[:, 1:]
    # left neighbor
    lowpoint[:, 1:] &= terrain[:, 1:] < terrain[:, :-1]
    return np.sum(terrain[lowpoint] + 1)


def solution2(terrain: np.ndarray) -> int:
    # The exercise states that only locations of height 9 are not part of any basin,
    # and every other location is part of only one basin. 
    # Therefore we decompose terrain into its contiguous compoments
    # by using the `label` function that finds and number contiguous features
    # of an array (i.e.  with the same value that are contiguous)
    basins, n = label(terrain != 9)
    sizes = sorted(np.sum(basins == i) for i in range(1, n+1))
    return np.prod(sizes[-3:]) # product of the top-3 basins


def main():
    with open("input.txt") as file:
        terrain = np.array([
            list(map(int, iter(line.strip()))) for line in file
        ])

    print(solution1(terrain))
    print(solution2(terrain))


if __name__ == "__main__":
    main()

import numpy as np


def main():
    with open("day13/input.txt") as file:
        raw = [[(c == '#') for c in line.strip()] for line in file]
    idx = [i for (i, row) in enumerate(raw) if not row]
    idx = [-1, *idx, -1]
    grids = [np.array(raw[start+1:stop]) for start, stop in zip(idx[:-1], idx[1:])]
    print(part1(grids))
    print("It doesn't work", part2(grids))


def part1(grids: list[np.ndarray]) -> int:
    total = 0
    for grid in grids:
        total += score1(grid)

    return total

def part2(grids: list[np.ndarray]) -> int:
    total = 0
    for grid in grids:
        total += score2(grid)
    return total

def score1(grid: np.ndarray) -> int:
    v, h = symmetries(grid)
    return 100*v if v is not None else (h or 0)

def score2(grid: np.ndarray) -> int:
    v, h = symmetries(grid)
    score = 100*v if v is not None else (h or 0)

    row, col = grid.shape
    for j in range(col):
        for i in range(row):
            grid[i, j] = not grid[i,j]
            new_v, new_h = symmetries(grid)
            grid[i, j] = not grid[i,j]
            new_score = 100*new_v if new_v is not None else (new_h or 0)
            if new_score != 0 and score != new_score:
                return new_score
    return 0

def symmetries(grid: np.ndarray) -> tuple[int|None, int|None]:
    row, col = grid.shape

    row_idx = None
    col_idx = None
    # horizontal symmetry
    for j in range(1, col):
        k = min(j, col-j)
        left = np.fliplr(grid[:, :j])
        right = grid[:, j:]
        if (left[:, :k] == right[:, :k]).all():
            col_idx = j
            break

    # vertical symmetry
    for i in range(1, row):
        k = min(i, row-i)
        up = np.flipud(grid[:i, :])
        down = grid[i:, :]
        if (up[:k, :] == down[:k, :]).all():
            row_idx = i
            break

    return row_idx, col_idx

if __name__ == "__main__":
    main()

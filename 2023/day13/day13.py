import numpy as np


def main():
    with open("day13/input.txt") as file:
        raw = [[(c == '#') for c in line.strip()] for line in file]
    idx = [i for (i, row) in enumerate(raw) if not row]
    idx = [-1, *idx, None]
    grids = [np.array(raw[start+1:stop]) for start, stop in zip(idx, idx[1:])]
    print(part1(grids))
    print(part2(grids))


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
    assert(len(v)+len(h)==1)
    score = 100*v[0] if v else h[0]
    return score

def score2(grid: np.ndarray) -> int:
    v, h = symmetries(grid)
    row, col = grid.shape
    for i in range(row):
        for j in range(col):
            grid[i, j] = not grid[i,j]
            new_v, new_h = symmetries(grid)
            grid[i, j] = not grid[i,j]

            for val in v:
                if val in new_v:
                    new_v.remove(val)
            for val in h:
                if val in new_h:
                    new_h.remove(val)
            assert(len(new_h) + len(new_v) <= 1)
            if len(new_h) + len(new_v) == 0:
                continue
            return 100*(new_v[0] if new_v else 0) + (new_h[0] if new_h else 0)
    return 0

def symmetries(grid: np.ndarray) -> tuple[list[int], list[int]]:
    row, col = grid.shape

    row_idx = []
    col_idx = []
    # horizontal symmetry
    for j in range(1, col):
        k = min(j, col-j)
        left = np.fliplr(grid[:, :j])
        right = grid[:, j:]
        if (left[:, :k] == right[:, :k]).all():
            col_idx.append(j)

    # vertical symmetry
    for i in range(1, row):
        k = min(i, row-i)
        up = np.flipud(grid[:i, :])
        down = grid[i:, :]
        if (up[:k, :] == down[:k, :]).all():
            row_idx.append(i)

    return row_idx, col_idx

if __name__ == "__main__":
    main()

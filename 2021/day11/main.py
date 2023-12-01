import numpy as np


def step(board: np.ndarray) -> np.ndarray:
    w, h = board.shape
    board += 1
    active = board > 9
    activated = np.zeros_like(board, dtype=bool)
    while active.any():
        lines, cols = np.nonzero(active)
        for x, y in zip(cols, lines):
            sx = slice(max(x - 1, 0), min(x + 2, w))
            sy = slice(max(y - 1, 0), min(y + 2, h))
            board[sy, sx] += 1
        activated |= active                 # all active becomes activated
        active = ~activated & (board > 9)   # get the newly active squids
    board[activated] = 0
    return board


def solution1(board: np.ndarray):
    flashes = 0
    for _ in range(100):
        board = step(board)
        flashes += np.sum(board == 0)
    return flashes


def solution2(board: np.ndarray):
    i = 0
    while not (board == 0).all():
        board = step(board)
        i += 1
    return i


def main():
    with open("input.txt") as file:
        board = np.array([[int(e) for e in line.strip()] for line in file])
    print(solution1(board.copy()))
    print(solution2(board.copy()))


if __name__ == "__main__":
    main()

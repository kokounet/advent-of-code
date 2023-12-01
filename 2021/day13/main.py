import numpy as np


def fold(board: np.ndarray, ax: str, coord: int) -> np.ndarray:
    if ax == 'x':
        newboard = board[:, :coord].copy()
        flip = np.fliplr(board[:, coord+1:])
    elif ax == 'y':
        newboard = board[:coord, :]
        flip = np.flipud(board[coord+1:, :])
    height, width = flip.shape
    newboard[-height:, -width:] |= flip
    return newboard


def solution1(board: np.ndarray, instructions: list[tuple[str, int]]):
    board = fold(board, *instructions[0])
    return np.sum(board)


def solution2(board: np.ndarray, instructions: list[tuple[str, int]]):
    for ax, coord in instructions:
        board = fold(board, ax, coord)
    return board


def display(board: np.ndarray):
    for line in board:
        for dot in line:
            print('█' if dot else ' ', end='')
        print()


def main():
    coords = []
    instructions = []
    with open("input.txt") as file:
        for line in file:
            if not line.strip():
                continue
            if line.startswith('fold along'):
                ax, coord = line.strip().removeprefix("fold along ").split("=")
                instructions.append((ax, int(coord)))
            else:
                coords.append(tuple(map(int, line.strip().split(','))))
    x, y = zip(*coords)
    w, h = max(x), max(y)
    board = np.zeros((h+1, w+1), dtype=bool)
    board[y, x] = True
    print(solution1(board.copy(), instructions))
    display(solution2(board.copy(), instructions))


if __name__ == "__main__":
    main()

import numpy as np


class Board:
    def __init__(self, data: list[list[int]]):
        n, m = len(data), len(data[0])
        self.numbers = {
            num: (l, c) for l, line in enumerate(data) for c, num in enumerate(line)
        }
        self.board = np.array(data)
        self.marks = np.zeros(shape=(n, m), dtype=bool)

    @property
    def won(self) -> bool:
        """Return true if any line or column is complete"""
        return (
            any(all(line) for line in self.marks) 
            or any(all(col) for col in self.marks.T)
        )
    
    @property
    def score(self) -> int:
        """Return the sum of the unmarked number of the board"""
        if not self.won:
            return 0
        return np.sum(self.board[~self.marks])
    
    def mark(self, number: int):
        """Mark a number on the board"""
        if number not in self.numbers:
            return
        line, col = self.numbers[number]
        self.marks[line, col] = True


def read_bingo(path) -> tuple[list[int], list[Board]]:
    with open(path) as file:
        it = map(str.strip, file)
        inputs = list(map(int, next(it).split(",")))
        next(it)
        boards, data = [], []
        for line in it:
            if not line:
                boards.append(Board(data))
                data.clear()
                continue
            data.append(list(map(int, line.split())))
    return inputs, boards

def solution01(path):
    inputs, boards = read_bingo(path)

    for number in inputs:
        for board in boards:
            board.mark(number)
            if board.won:
                return board.score * number


def solution02(path):
    inputs, boards = read_bingo(path)

    for number in inputs:
        for board in boards:
            board.mark(number)
        if len(boards) == 1 and boards[0].won:
            return number * boards[0].score
        boards = [board for board in boards if not board.won]
        

def main():
    print(solution01("input.txt"))
    print(solution02("input.txt"))


if __name__ == "__main__":
    main()

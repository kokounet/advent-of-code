import numpy as np

from collections import defaultdict
from heapq import heapify, heappush, heappop


def neighbors(u: tuple, dim: tuple):
    i, j = u
    lines, cols = dim
    if i < lines - 1:
        yield i + 1, j
    if j < cols - 1:
        yield i, j + 1
    if i > 0:
        yield i - 1, j
    if j > 0: 
        yield i, j - 1


def shortest_path(dim: tuple, source: tuple, target: tuple, dist_func):
    prev = {(i, j): None for i in range(dim[0]) for j in range(dim[1])}
    dist = defaultdict(lambda: np.inf)
    dist[source] = 0
    visited = set()
    Q = [(0, source)]
    while Q:
        w, u = heappop(Q)
        if u == target:
            break
        if u in visited:
            continue
        visited.add(u)

        for v in neighbors(u, dim):
            alt = dist[u] + dist_func(v)
            if alt < dist[v]:
                dist[v] = alt
                prev[v] = u
                heappush(Q, (alt, v))

    path = [target]
    while prev[path[-1]] != source:
        path.append(prev[path[-1]])

    return path, dist[target]



def solution1(board):
    lines, cols = board.shape
    _, dist = shortest_path(
        dim=(lines, cols), 
        source=(0, 0), 
        target=(lines-1, cols-1), 
        dist_func=lambda v: board[v]
    )
    return int(dist)


def solution2(board):
    lines, cols = board.shape
    def dist_func(v):
        i, j = v
        return (board[i%lines, j%cols] + i // lines + j // cols - 1) % 9 + 1

    _, dist = shortest_path(
        dim=(5*lines, 5*cols),
        source=(0, 0),
        target=(5*lines-1, 5*cols-1),
        dist_func=dist_func
    )
    return int(dist)


def main():
    with open("input.txt") as file:
        board = np.array([
            list(map(int, line.strip())) for line in file 
        ])
    print(solution1(board.copy()))
    print(solution2(board.copy()))


if __name__ == "__main__":
    main()

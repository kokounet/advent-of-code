import numpy as np
from collections import Counter, deque


def init(counter: Counter) -> deque:
    population = deque((0 for _ in range(9)), maxlen=9)
    for age, count in counter.items():
        population[age] = count
    return population


def simulate(population: deque, days: int) -> deque:
    for _ in range(days):
        population.rotate(-1)
        population[6] += population[8]
    return population.copy()


def np_init(counter: Counter) -> np.ndarray:
    population = np.zeros((9,), dtype=np.int64)
    for age, count in counter.items():
        population[age] = count
    return population


def np_simulate(population: np.ndarray, days: int) -> np.ndarray:
    matrix = np.eye(9, k=1, dtype=np.int64)
    matrix[[6, 8], 0] = 1
    return np.linalg.matrix_power(matrix, days) @ population


def solution1(counter):
    population = np_simulate(np_init(counter), 80)
    return sum(population)
    

def solution2(counter):
    population = np_simulate(np_init(counter), 256)
    return sum(population)


def main():
    with open("input.txt") as file:
        counter = Counter(map(int, next(file).strip().split(",")))
    print(solution1(counter))
    print(solution2(counter))
    

if __name__ == "__main__":
    main()

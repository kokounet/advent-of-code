import numpy as np


def solution01(file):
    data = np.loadtxt(file)
    return np.sum(np.diff(data) > 0)


def solution02(file):
    data = np.loadtxt(file)
    filtered = np.convolve(data, [1, 1, 1], mode='valid')
    return np.sum(np.diff(filtered) > 0)


if __name__ == '__main__':
    print(solution01("input.txt"))
    print(solution02("input.txt"))

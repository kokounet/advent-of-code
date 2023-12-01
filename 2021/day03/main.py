import numpy as np
from collections import Counter
from functools import reduce


def to_int(bits):
    return reduce(lambda acc, val: (acc << 1) | int(val), bits, 0)


def solution1(path):
    with open(path) as file:
        diagnostic = zip(*[[bool(int(c)) for c in line.strip()] for line in file])
    gamma = [Counter(seq).most_common(1)[0][0] for seq in diagnostic]
    epsilon = [not b for b in gamma]
    return to_int(gamma) * to_int(epsilon)


def solution2(path): 
    with open(path) as file:
        diagnostic = np.array([[bool(int(c)) for c in line.strip()] for line in file])

    def filtering(array, mask, compfunc):
        masked = array[mask]
        ones = sum(masked)
        zeros = len(masked) - ones
        return array if compfunc(ones, zeros) else ~array

    oxygen_mask = np.ones(diagnostic.shape[0], dtype=bool)
    carbon_mask = np.ones(diagnostic.shape[0], dtype=bool)
    for bits in diagnostic.T:
        # update the masks if we haven't found the number yet
        if sum(oxygen_mask) > 1:
            oxygen_mask &= filtering(bits, oxygen_mask, lambda x, y: x >= y)
        if sum(carbon_mask) > 1:
            carbon_mask &= filtering(bits, carbon_mask, lambda x, y: x < y)
    
    oxygen_rating = to_int(diagnostic[oxygen_mask, :][0])
    carbon_rating = to_int(diagnostic[carbon_mask, :][0])
    return oxygen_rating * carbon_rating

def main():
    print(solution1("input.txt"))
    print(solution2("input.txt"))

if __name__ == "__main__":
    main()

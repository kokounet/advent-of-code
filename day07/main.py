from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path


def main():
    available = 70_000_000
    update = 30_000_000
    with Path("day07/input.txt").open() as file:
        lines = [l.strip().split() for l in file]

    files = {}
    dirs = defaultdict(lambda: 0)
    cwd = ["/"]
    for line in lines:
        match line:
            case ["$", "cd", ".."]:
                cwd.pop()
            case ["$", "cd", "/"]:
                cwd = ["/"]
            case ["$", "cd", dir]:
                cwd.append(dir)
            case [ssize, name] if ssize not in ["$", "dir"]:
                size = int(ssize)
                available -= size
                files[tuple([*cwd, name])] = size
                for i in range(len(cwd)):
                    dirs[tuple(cwd[: i + 1])] += size

    print(sum(filter(lambda size: size < 100000, dirs.values())))
    print(min(filter(lambda size: (available + size) > update, dirs.values())))


if __name__ == "__main__":
    main()

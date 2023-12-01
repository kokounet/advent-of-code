import pandas as pd


def solution01():
    df = pd.read_csv("input.txt", sep=' ', header=None)
    df.columns = ['direction', 'amount']
    groups = df.groupby('direction').sum()
    print((groups.at['down', 'amount'] - groups.at['up', 'amount'])*groups.at['forward', 'amount'])


if __name__ == '__main__':
    solution01()

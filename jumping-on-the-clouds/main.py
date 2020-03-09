#!/bin/python3

def jumpingOnClouds(c):
    current = 0
    count = 0

    while True:
        current += 2 if current + 2 < len(c) and c[current + 2] == 0 else 1
        if current >= len(c):
            return count
        else:
            count += 1

if __name__ == '__main__':
    n = int(input())
    c = list(map(int, raw_input().rstrip().split()))
    r = jumpingOnClouds(c)
    print(r)
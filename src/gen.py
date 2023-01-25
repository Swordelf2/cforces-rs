import random

print(7 * 1000)
res = []
for n in range(1, 8):
    for t in range(1000):
        m = random.randrange(1, 1000)
        print(n, m)
        for _ in range(n):
            print(random.randrange(1, 1000), end=' ')
        print()


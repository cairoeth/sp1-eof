@external
@pure
def fib(n: uint32) -> uint32:
    if n == 0:
        return 0

    a: uint32 = 1
    b: uint32 = 1

    if 2 < n:
        for i: uint32 in range(2, n, bound = 1000):
            c: uint32 = (a + b) % 7919
            a = b
            b = c

    d: uint32 = b

    return d
import itertools

import numpy as np


def accum_unordered_vec(x):
    y = np.zeros(x.shape)
    for ix in itertools.permutations(range(5), 5):
        y += np.moveaxis(x, [0, 1, 2, 3, 4], ix)

    z = np.zeros(x.shape)
    for ix in itertools.combinations(range(x.shape[0]), 5):
        z[ix] = y[ix]

    return z


def accum_unordered(x):
    g = np.zeros(x.shape)
    for ix in itertools.permutations(range(x.shape[0]), 5):
        g[tuple(sorted(ix))] += x[ix]

    return g


def fast_sort5(a, b, c, d, e):
    "Sort 5 values with 7 Comparisons"
    if a < b:
        a, b = b, a
    if c < d:
        c, d = d, c
    if a < c:
        a, b, c, d = c, d, a, b
    if e < c:
        if e < d:
            pass
        else:
            d, e = e, d
    else:
        if e < a:
            c, d, e = e, c, d
        else:
            a, c, d, e = e, a, c, d
    if b < d:
        if b < e:
            return b, e, d, c, a
        else:
            return e, b, d, c, a
    else:
        if b < c:
            return e, d, b, c, a
        else:
            return e, d, c, b, a


def accum_unordered_loop(x):
    n = x.shape[0]
    g = np.zeros(x.shape)

    for i in range(n):
        for j in range(n):
            if i != j:
                for k in range(n):
                    if i != k and j != k:
                        for l in range(n):
                            if i != l and j != l and k != l:
                                for m in range(n):
                                    if i != m and j != m and k != m and l != m:
                                        ix, jx, kx, lx, mx = fast_sort5(i, j, k, l, m)
                                        g[ix, jx, kx, lx, mx] += x[i, j, k, l, m]

    return g

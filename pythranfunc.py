import numpy as np


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


#pythran export pthr_accum_unordered(float64[:,:,:,:,:] order(C))
def pthr_accum_unordered(x):
    n = x.shape[0]
    g = np.zeros(x.shape)

    for i in range(n):
        for j in range(n):
            if i == j:
                continue
            for k in range(n):
                if i == k or j == k:
                    continue
                for l in range(n):
                    if i == l or j == l or k == l:
                        continue
                    for m in range(n):
                        if i != m and j != m and k != m and l != m:
                            ix, jx, kx, lx, mx = fast_sort5(i, j, k, l, m)
                            g[ix, jx, kx, lx, mx] += x[i, j, k, l, m]

    return g

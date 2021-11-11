# cython: language_level=3, boundscheck=False

import numpy as np
cimport numpy as np


cdef void fast_sort5(int a, int b, int c, int d, int e,
        int* ao, int* bo, int* co, int* do, int* eo):
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
            ao[0] = b
            bo[0] = e
            co[0] = d
            do[0] = c
            eo[0] = a
        else:
            ao[0] = e
            bo[0] = b
            co[0] = d
            do[0] = c
            eo[0] = a
    else:
        if b < c:
            ao[0] = e
            bo[0] = d
            co[0] = b
            do[0] = c
            eo[0] = a
        else:
            ao[0] = e
            bo[0] = d
            co[0] = c
            do[0] = b
            eo[0] = a


cpdef np.float64_t[:, :, :, :, ::1] cy_accum_unordered(np.float64_t[:, :, :, :, ::1] x):
    cdef:
        int i, j, k, l, m, n
        int ix, jx, kx, lx, mx

        np.ndarray[np.float64_t, ndim=5] gout
        np.float64_t[:, :, :, :, ::1] g

    n = x.shape[0]
    gout = np.zeros_like(x)
    g = gout

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
                            fast_sort5(i, j, k, l, m, &ix, &jx, &kx, &lx, &mx)
                            g[ix, jx, kx, lx, mx] += x[i, j, k, l, m]

    return gout

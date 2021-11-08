use std::mem;

use itertools::Itertools;

use numpy::ndarray::{Array, Array5, ArrayView5, Axis};
use numpy::{IntoPyArray, PyArray5, PyReadonlyArray5};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

fn fast_sort5(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
) -> (usize, usize, usize, usize, usize) {
    //Sort 5 values with 7 Comparisons
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut d = d;
    let mut e = e;

    if a < b {
        mem::swap(&mut a, &mut b);
    }

    if c < d {
        mem::swap(&mut c, &mut d);
    }

    if a < c {
        mem::swap(&mut a, &mut c);
        mem::swap(&mut b, &mut d);
    }

    if e < c {
        if e < d {
            ();
        } else {
            mem::swap(&mut d, &mut e);
        }
    } else {
        if e < a {
            let ctmp = c;
            let dtmp = d;
            let etmp = e;
            c = etmp;
            d = ctmp;
            e = dtmp;
        } else {
            let atmp = a;
            let ctmp = c;
            let dtmp = d;
            let etmp = e;
            a = etmp;
            c = atmp;
            d = ctmp;
            e = dtmp;
        }
    }

    if b < d {
        if b < e {
            return (b, e, d, c, a);
        } else {
            return (e, b, d, c, a);
        }
    } else {
        if b < c {
            return (e, d, b, c, a);
        } else {
            return (e, d, c, b, a);
        }
    }
}

fn _accum_unordered1(x: ArrayView5<f64>) -> Array5<f64> {
    let n = x.len_of(Axis(0));
    let mut g = Array::zeros(x.raw_dim());

    for i in 0..n {
        for j in 0..n {
            if i != j {
                for k in 0..n {
                    if i != k && j != k {
                        for l in 0..n {
                            if i != l && j != l && k != l {
                                for m in 0..n {
                                    if i != m && j != m && k != m && l != m {
                                        let (ix, jx, kx, lx, mx) = fast_sort5(i, j, k, l, m);
                                        g[[ix, jx, kx, lx, mx]] += x[[i, j, k, l, m]];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    g
}

fn _accum_unordered2(x: ArrayView5<f64>) -> Array5<f64> {
    let n = x.len_of(Axis(0));
    let mut g = Array::zeros(x.raw_dim());
    let mut sx: [usize; 5];

    for i in 0..n {
        for j in 0..n {
            if i != j {
                for k in 0..n {
                    if i != k && j != k {
                        for l in 0..n {
                            if i != l && j != l && k != l {
                                for m in 0..n {
                                    if i != m && j != m && k != m && l != m {
                                        sx = [i, j, k, l, m];
                                        sx.sort_unstable();
                                        g[sx] += x[[i, j, k, l, m]];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    g
}

fn _accum_unordered3(x: ArrayView5<f64>) -> Array5<f64> {
    let n = x.len_of(Axis(0));
    let mut g = Array::zeros(x.raw_dim());
    for ix in (0..n).permutations(5) {
        let (i, j, k, l, m) = (ix[0], ix[1], ix[2], ix[3], ix[4]);
        let (ix, jx, kx, lx, mx) = fast_sort5(i, j, k, l, m);
        g[[ix, jx, kx, lx, mx]] += x[[i, j, k, l, m]];
    }

    g
}

fn _accum_unordered4(x: ArrayView5<f64>) -> Array5<f64> {
    let mut g = Array::zeros(x.raw_dim());
    for ((i, j, k, l, m), elt) in x.indexed_iter() {
        if i == j
            || i == k
            || i == l
            || i == m
            || j == k
            || j == l
            || j == m
            || k == l
            || k == m
            || l == m
        {
            continue;
        }
        let (ix, jx, kx, lx, mx) = fast_sort5(i, j, k, l, m);
        g[[ix, jx, kx, lx, mx]] += elt;
    }

    g
}

fn _accum_unordered5(x: ArrayView5<f64>) -> Array5<f64> {
    let n = x.len_of(Axis(0));
    let mut g = Array::zeros(x.raw_dim());

    for i in 0..n {
        for j in 0..n {
            if i != j {
                for k in 0..n {
                    if i != k && j != k {
                        for l in 0..n {
                            if i != l && j != l && k != l {
                                for m in 0..n {
                                    if i != m && j != m && k != m && l != m {
                                        let (ix, jx, kx, lx, mx) = fast_sort5(i, j, k, l, m);
                                        unsafe {
                                            *g.uget_mut([ix, jx, kx, lx, mx]) +=
                                                x.uget([i, j, k, l, m]);
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    g
}

#[pymodule]
#[pyo3(name = "_rsfunc")]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // wrapper of `_accum_unordered1`
    #[pyfn(m)]
    #[pyo3(name = "rs_accum_unordered1")]
    fn accum_unordered1_py<'py>(py: Python<'py>, x: PyReadonlyArray5<f64>) -> &'py PyArray5<f64> {
        let x = x.as_array();
        _accum_unordered1(x).into_pyarray(py)
    }

    // wrapper of `_accum_unordered2`
    #[pyfn(m)]
    #[pyo3(name = "rs_accum_unordered2")]
    fn accum_unordered2_py<'py>(py: Python<'py>, x: PyReadonlyArray5<f64>) -> &'py PyArray5<f64> {
        let x = x.as_array();
        _accum_unordered2(x).into_pyarray(py)
    }

    // wrapper of `_accum_unordered3`
    #[pyfn(m)]
    #[pyo3(name = "rs_accum_unordered3")]
    fn accum_unordered3_py<'py>(py: Python<'py>, x: PyReadonlyArray5<f64>) -> &'py PyArray5<f64> {
        let x = x.as_array();
        _accum_unordered3(x).into_pyarray(py)
    }

    // wrapper of `_accum_unordered4`
    #[pyfn(m)]
    #[pyo3(name = "rs_accum_unordered4")]
    fn accum_unordered4_py<'py>(py: Python<'py>, x: PyReadonlyArray5<f64>) -> &'py PyArray5<f64> {
        let x = x.as_array();
        _accum_unordered4(x).into_pyarray(py)
    }

    // wrapper of `_accum_unordered5`
    #[pyfn(m)]
    #[pyo3(name = "rs_accum_unordered5")]
    fn accum_unordered5_py<'py>(py: Python<'py>, x: PyReadonlyArray5<f64>) -> &'py PyArray5<f64> {
        let x = x.as_array();
        _accum_unordered5(x).into_pyarray(py)
    }

    Ok(())
}

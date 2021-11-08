# A small comparison between different methods of accelerating numerical python code


To get started, install the required packages. I used `conda`:

```bash
$ conda create -n ndarray_comparison python=3.9 numpy numba=0.54.1 cython jupyterlab seaborn pythran watermark setuptools-rust --override-channels -c conda-forge

conda activate ndarray_comparison
```

Then to compile the extensions:

```bash
$ python setup.py build_ext --inplace
```

The benchmark can be found in the following ipython notebook:
https://github.com/synapticarbors/ndarray_comparison/blob/main/comparison.ipynb


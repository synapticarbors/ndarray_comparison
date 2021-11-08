from setuptools import setup
from setuptools_rust import RustExtension
from Cython.Build import cythonize
from pythran.dist import PythranExtension

import numpy as np

pythran_ext = PythranExtension('pythranfunc', sources=['pythranfunc.py'])

setup(
    name="My hello app",
    ext_modules=cythonize('cyfunc.pyx') + [pythran_ext, ],
    rust_extensions=[
        RustExtension(
            "rust_ext._rsfunc",
            debug=False,
        ),
    ],
    include_dirs=[np.get_include()],
)

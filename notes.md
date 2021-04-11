# Notes on interfacing python and Rust

A great article: https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/

The Rust module will have to be to the target system or embed binaries in Python wheels.

`On Windows, you will need to rename the output from *.dll to *.pyd.`

To use a specific python interpreter: set PYTHON_SYS_EXECUTABLE environment variable to it.


## Rust crates for interfacing

- https://github.com/dgrunwald/rust-cpython

- https://github.com/PyO3/pyo3

- https://github.com/getsentry/milksnake

- https://pypi.org/project/setuptools-rust/

## Python packages for interfacing

https://pypi.org/project/rustypy/





## building

    cargo build --release

    

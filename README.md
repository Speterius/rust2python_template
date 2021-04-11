# rust2python_template

This repository is a template to create Rust bindings for Python. This example creates a python module called `hello_rust` with a function `add(a, b)` that adds two integers.

## Setup

Clone the repository.

Do not use `venv`, because the `setuptools-rust` package will not set the correct python lib folder and compilation and linking will fail.

Make sure you have the packages installed, from `requirements.txt`

Make sure you have Rust and cargo installed.

## Usage

Check if it compiles properly:

    cargo build --release

Use the setuptools to make a python package:

    python setup.py develop
    
Run the unit tests in python


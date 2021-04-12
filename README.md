# rust2python_template

This repository is a template to create Rust bindings for Python. 

This example creates a python module called `hello_rust` with a function `add(a, b)` that adds two 
integers, and a class called `Rectangle(w, h)` with fields `width` and `height` and a method called `area()`.

## Setup

0) Install Rust and `cargo`

1) Clone the repository.

2) `python -m venv venv`

3) `venv\scripts\activate` or `source venv/bin/activate`

4) `pip install -r requirements.txt`

## Usage

Maturin for development:

    maturin develop
    
Run the unit tests:

    pytest

Distribute the `wheel` file using:

    maturin build


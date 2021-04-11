from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="hello-rust",
    version="0.1",
    rust_extensions=[RustExtension("hello_rust.hello_rust", binding=Binding.RustCPython)],
    packages=["hello_rust"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)

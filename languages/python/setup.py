import pathlib
from setuptools import setup

# The directory containing this file
HERE = pathlib.Path(__file__).parent

# The text of the README file
README = (HERE / "README.md").read_text()

# This call to setup() does all the work
setup(
    name="memquery",
    version="1.0.0",
    description="MemQuery is simple library for creating, querying, and updating in memory documents that are represented as JSON objects and queried using Mongodb like operators.",
    long_description=README,
    long_description_content_type="text/markdown",
    url="https://github.com/robjsliwa/mem_query",
    author="Rob Sliwa",
    author_email="robjsliwa@gmail.com",
    license="MIT",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.6",
        "Topic :: inmemory",
        "Topic :: query",
        "Topic:: database",
        "Topic :: json",
        "Topic :: nosql"
    ],
    packages=["memquery"],
    include_package_data=True,
    install_requires=["feedparser", "html2text"],
    entry_points="",
)
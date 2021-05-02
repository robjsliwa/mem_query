import pathlib
import os
from setuptools import setup

# The directory containing this file
HERE = pathlib.Path(__file__).parent

# The text of the README file
README = (HERE / "README.md").read_text()

NAME = 'memquery'
VERSION = None

project_path = os.path.abspath(os.path.dirname(__file__))

about = {}
if not VERSION:
    project_slug = NAME.lower().replace("-", "_").replace(" ", "_")
    with open(os.path.join(project_path, project_slug, "__version__.py")) as f:
        exec(f.read(), about)
else:
    about["__version__"] = VERSION

# This call to setup() does all the work
setup(
    name="memquery",
    version=about["__version__"],
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
        "Programming Language :: Python :: 3.7"
    ],
    keywords = "inmemory query database json nosql",
    packages=["memquery", "errors", "membind"],
    include_package_data=True,
    install_requires=["wasmtime==0.25.0"],
    entry_points="",
)

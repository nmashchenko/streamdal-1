from setuptools import setup
from pathlib import Path

this_directory = Path(__file__).parent
long_description = (this_directory / "README.md").read_text()
setup(
    name="streamdal",
    version='0.1.2',
    description="Python client SDK for Streamdal's open source observability server",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/streamdal/python-sdk",
    author="Streamdal.com",
    author_email="engineering@streamdal.com",
    license="MIT",
    packages=[
        "streamdal",
        "streamdal.common",
        "streamdal.kv",
        "streamdal.metrics",
        "streamdal.validation",
        "streamdal.tail",
        "streamdal.hostfunc",
    ],
    install_requires=[
        "betterproto==2.0.0b6",
        "black==23.11.0",
        "bleach==6.1.0",
        "certifi==2023.11.17",
        "cffi==1.16.0",
        "charset-normalizer==3.3.2",
        "click==8.1.7",
        "coverage==7.3.2",
        "docutils==0.20.1",
        "grpcio==1.59.3",
        "grpclib==0.4.6",
        "h2==4.1.0",
        "hpack==4.0.0",
        "hyperframe==6.0.1",
        "idna==3.6",
        "importlib-metadata==6.8.0",
        "iniconfig==2.0.0",
        "isort==5.12.0",
        "jaraco.classes==3.3.0",
        "Jinja2==3.1.3",
        "keyring==24.3.0",
        "markdown-it-py==3.0.0",
        "MarkupSafe==2.1.3",
        "mdurl==0.1.2",
        "more-itertools==10.1.0",
        "multidict==6.0.4",
        "mypy-extensions==1.0.0",
        "nh3==0.2.14",
        "packaging==23.2",
        "pathspec==0.11.2",
        "pkginfo==1.9.6",
        "platformdirs==4.0.0",
        "pluggy==1.3.0",
        "protobuf==4.25.1",
        "pycparser==2.21",
        "Pygments==2.17.2",
        "pytest==7.4.3",
        "pytest-asyncio==0.21.1",
        "pytest-cov==4.1.0",
        "pytest-mock==3.12.0",
        "python-dateutil==2.8.2",
        "readme-renderer==42.0",
        "requests==2.31.0",
        "requests-toolbelt==1.0.0",
        "rfc3986==2.0.0",
        "rich==13.7.0",
        "six==1.16.0",
        "streamdal-protos==0.1.18",
        "stringcase==1.2.0",
        "tf==1.0.0",
        "token-bucket==0.3.0",
        "twine==4.0.2",
        "urllib3==2.1.0",
        "wasmer==1.1.0",
        "wasmer-compiler-cranelift==1.1.0",
        "wasmtime==15.0.0",
        "webencodings==0.5.1",
        "zipp==3.17.0",
    ],
    python_requires=">=3.8",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
    ],
)

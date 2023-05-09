Python GET request written in rust
---


* ``python -m venv venv``
* ``source venv/bin/activate``
* ``pip install maturin``
* ``pip install patchelf``
* ``maturin build --release``
* ``pip install <a library that was built into maturin>``
* ``python ./examples/main.py``

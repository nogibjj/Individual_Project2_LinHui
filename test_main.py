"""
Test goes here

"""

from mylib.calculator import add


def test_add():
    assert add(1, 2) == 3


def test_numpy():
    import numpy as np
    assert np.__version__ == '1.25.1'

def test_pandas():
    import pandas as pd
    assert pd.__version__ == '2.1.0'

#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import math
from matplotlib import pyplot as plt


def f(fovy, aspect):
    '''
    Return the fov across x for given aspect.
    '''
    x = math.tan(fovy) * aspect
    return math.atan(x)


items = lambda xs, i: [x[i] for x in xs]
steps = lambda xs: [x / xs[-1] for x in xs]
# flat = lambda xs: [items(x, 0) for x in xs]


def plot2d(seq, *args):
    '''Plot in two dimensions.'''
    plt.plot(items(seq, 0), items(seq, 1), *args)


def main():
    # xs = steps(range(0, 1000))
    xs = range(0, 91)
    ys = [f(math.pi * x / 180, 2) * 180 / math.pi for x in xs]
    plt.plot(xs, ys, 'r--')
    plt.show()


if __name__ == '__main__':
    main()

import sys

from ctypes import *

class _Coord(Structure):
    _fields_ = [ ("latitude", c_double),
                 ("longitude", c_double) ]

lib = CDLL('target/debug/libpolyline_ffi.so')

def encode_coordinates(seq):
    fun = lib.encode_coordinates_ffi2
    fun.argtypes = (c_char_p, c_size_t, c_void_p, c_size_t)
    fun.restype = c_int

    arr = (_Coord * len(seq))()

    for i, (lat, long_) in enumerate(seq):
        arr[i].latitude = lat
        arr[i].longitude = long_

    buf = create_string_buffer(256)
    res = fun(buf, 256, arr, len(seq))
    return buf.value

def decode_polyline(pline, count=5):
    fun = lib.decode_polyline_ffi
    fun.argtypes = (c_void_p, POINTER(c_size_t), c_char_p)
    fun.restype = c_int

    coords = (_Coord * count)()
    count = c_size_t(count)
    res = fun(coords, byref(count), pline)
    # print('Res: {:d} {:d}'.format(res, count.value))
    if res < 0:
        # call failed due to insufficient array size, required size also returned
        coords = (_Coord * count.value)()
        count = c_size_t(count.value)
        res = fun(coords, byref(count), pline)
        # print('Res: {:d} {:d}'.format(res, count.value))

    seq = []
    for i in range(count.value):
        seq.append((coords[i].latitude, coords[i].longitude))

    return seq

class _FFIArray(Structure):
    _fields_ = [("data", c_void_p), ("len", c_size_t)]

class Coordinate(Structure):
    _fields_ = [("latitude", c_double), ("longitude", c_double)]

class Coordinates(Structure):
    _fields_ = [("data", POINTER(Coordinate)), ("len", c_size_t)]

import numpy as np
from numpy.ctypeslib import ndpointer

def decode_polyline2(pline, count=5):
    fun = lib.decode_polyline_ffi2
    fun.argtypes = [c_char_p]
    fun.restype = _FFIArray

    res = fun(pline)
    cs = Coordinates(cast(res.data, POINTER(Coordinate)), res.len)

    for i in range(cs.len):
        print(cs.data[i].latitude, cs.data[i].longitude)

    xs = np.ctypeslib.as_array((c_double * res.len * 2).from_address(res.data))
    xs = xs.reshape((res.len, 2))
    print(xs)

    lib.free_coordinates(res.data)

import random as r

seq = []
a = 60.0
b = 150.0

for i in range(10):
    seq.append((a, b))
    a += r.uniform(-0.5, 0.5)
    b += r.uniform(-10.0, 10.0)

import math
res = encode_coordinates(seq)
print(res, len(res))
seq2 = decode_polyline(res, int(math.ceil(len(res) / 10.0)))
print(seq, seq2)
print('All ok? ', all(abs(x[0] - y[0]) < 0.00001 and abs(x[1] - y[1]) < 0.00001 for x, y in zip(seq, seq2)))

decode_polyline2(res)

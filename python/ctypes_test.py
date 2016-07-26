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

seq = [[1.0, 2.0], [3.0, 4.0]]
res = encode_coordinates(seq)
print(res)

extern crate libc;
extern crate polyline;

use libc::*;
use std::{ptr, slice};
use std::ffi::*;

use polyline::*;

#[no_mangle]
pub extern "C" fn free_cstring(p: *mut c_char) {
    // println!("{:p} pointer", p);
    unsafe { CString::from_raw(p) };
}

#[no_mangle]
pub extern "C" fn free_coordinates(p: *mut c_void) {
    // println!("{:p} pointer", p);
    unsafe { Box::from_raw(p) };
}

#[repr(C)]
pub struct Coord {
    pub latitude: f64,
    pub longitude: f64
}

#[repr(C)]
pub struct Coordinates {
    pub data: *const c_void,
    pub nmemb: usize
}

#[no_mangle]
pub extern "C" fn encode_coordinates_ffi(points: *mut Coord,
                                         npoints: usize) -> *mut c_char {
    let mut cs: Vec<[f64; 2]> = Vec::new();

    unsafe {
	    let slice = slice::from_raw_parts_mut(points, npoints);
	    for coord in slice {
	        cs.push([coord.latitude, coord.longitude]);
	    }
    };

    let tuple = match encode_coordinates(&cs, 5) {
	    Ok(res) => (0, res),
	    Err(res) => (-1, res)
    };
    let (_rc, result) = tuple;
    CString::new(result).unwrap().into_raw()
}

fn coord_vec_to_c(res: Vec<[f64; 2]>, points: *mut Coord, npoints: *mut usize) -> c_int {
    unsafe {
        if res.len() > *npoints {
            *npoints = res.len();
            return -1;
        }
    }

    let len = res.len();
    let slice = unsafe {
        slice::from_raw_parts_mut(points, *npoints)
    };

    for i in 0..len as usize {
        let pair = res[i];
        slice[i].latitude = pair[0];
        slice[i].longitude = pair[1];
    }

    unsafe { *npoints = res.len(); }
    0
}

#[no_mangle]
pub extern "C" fn decode_polyline_ffi(points: *mut Coord,
                                      npoints: *mut usize,
                                      src: *const c_char) -> c_int {
    let s: String = unsafe {
        CStr::from_ptr(src).to_string_lossy().into_owned()
    };

    let res = decode_polyline(s, 8);

    match res {
        Ok(vec) => coord_vec_to_c(vec, points, npoints),
        Err(_) => -1,
    }
}

fn coord_vec_to_c2(coordinates: Vec<[f64; 2]>) -> Coordinates {
    let len: usize = coordinates.len();

    let data = coordinates.into_boxed_slice();
    let raw = Box::into_raw(data);

    // println!("{:p} pointer", raw);
    let res: Coordinates = Coordinates{nmemb: len, data: raw as *const c_void};
    res
}

#[no_mangle]
pub extern "C" fn decode_polyline_ffi2(src: *const c_char) -> Coordinates {
    let s: String = unsafe {
        CStr::from_ptr(src).to_string_lossy().into_owned()
    };

    let res = decode_polyline(s, 5);

    match res {
        Ok(vec) => coord_vec_to_c2(vec),
        Err(_) => Coordinates{nmemb: 0, data: ptr::null_mut()},
    }
}

#[no_mangle]
pub extern "C" fn encode_coordinates_ffi2(dst: *mut c_char,
                                          n: usize,
                                          points: *mut Coord,
                                          npoints: usize) -> c_int {
    let mut cs: Vec<[f64; 2]> = Vec::new();
    let slice = unsafe { slice::from_raw_parts_mut(points, npoints) };

	for coord in slice {
	    cs.push([coord.latitude, coord.longitude]);
    };

    let (rc, result) = match encode_coordinates(&cs, 5) {
	    Ok(res) => (0, res),
	    Err(res) => (-1, res)
    };

    if rc < 0 || result.len() >= n {
	    return -1;
    }

    let mut i: usize = 0;
    let slice2 = unsafe { slice::from_raw_parts_mut(dst, n) };

	for c in result.chars() {
	    slice2[i] = c as i8;
	    i = i + 1;
	}
	slice2[i] = 0;
    0
}

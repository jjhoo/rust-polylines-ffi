extern crate libc;
extern crate polyline;

use libc::*;
use std::slice;
use std::ffi::*;

use polyline::*;

#[no_mangle]
pub extern "C" fn free_cstring(p: *mut c_char) {
    // println!("{:p} pointer", p);
    unsafe { CString::from_raw(p) };
}

#[repr(C)]
pub struct Coord {
    pub latitude: f64,
    pub longitude: f64
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

#[no_mangle]
pub extern "C" fn encode_coordinates_ffi2(dst: *mut c_char,
                                          n: usize,
                                          points: *mut Coord,
                                          npoints: usize) -> c_int {
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
    let (rc, result) = tuple;

    if rc < 0 || result.len() >= n {
	    return -1;
    }

    let mut i: usize = 0;
    unsafe {
	    let slice2 = slice::from_raw_parts_mut(dst, n);
	    for c in result.chars() {
	        slice2[i] = c as i8;
	        i = i + 1;
	    }
	    slice2[i] = 0;
    };
    0
}

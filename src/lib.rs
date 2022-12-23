/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use vin_parser::check_validity;
use vin_parser::verify_checksum;
use vin_parser::get_info;
use vin_parser::VINError;

#[repr(C)]
struct vinparser_checksum_error_info_t {
    expected: c_char,
    received: c_char,
}

#[repr(C)]
struct vinparser_vin_t {
    vin: *mut c_char,
    country: *mut c_char,
    manufacturer: *mut c_char,
    region: *mut c_char,
    valid_checksum: c_int,
    checksum_error_info: vinparser_checksum_error_info_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum vinparser_error_t {
    VINPARSER_ERROR_INCORRECTLENGTH,
    VINPARSER_ERROR_INVALIDCHARACTERS,
    VINPARSER_ERROR_CHECKSUMERROR,
}

#[no_mangle]
unsafe extern "C" fn vinparser_check_validity(vin: *const c_char, error_code: *mut vinparser_error_t) -> c_int {
    let vin_rs = match CStr::from_ptr(vin).to_str() {
	Ok(str) => str,
	Err(_) => return -1
    };
    match check_validity(vin_rs) {
	Ok(_) => 0,
	Err(err) => match err {
	    VINError::IncorrectLength => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_INCORRECTLENGTH;
		-1
	    },
	    VINError::InvalidCharacters(_) => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_INVALIDCHARACTERS;
		-1
	    },
	    VINError::ChecksumError(_) => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_CHECKSUMERROR;
		-1
	    }
	}
    }
}

#[no_mangle]
unsafe extern "C" fn vinparser_get_info(vin: *const c_char, vinparser_vin: *mut vinparser_vin_t, error_code: *mut vinparser_error_t) -> c_int {
    let vin_rs = match CStr::from_ptr(vin).to_str() {
	Ok(str) => str,
	Err(_) => return -1
    };
    match get_info(vin_rs) {
	Ok(info) => {
	    let info_vin = match CString::new(info.vin) {
		Ok(cstr) => cstr.into_raw(),
		Err(_) => ptr::null_mut()
	    };
	    let info_country = match CString::new(info.country) {
		Ok(cstr) => cstr.into_raw(),
		Err(_) => ptr::null_mut()
	    };
	    let info_manufacturer = match CString::new(info.manufacturer) {
		Ok(cstr) => cstr.into_raw(),
		Err(_) => ptr::null_mut()
	    };
	    let info_region = match CString::new(info.region) {
		Ok(cstr) => cstr.into_raw(),
		Err(_) => ptr::null_mut()
	    };
	    let info_valid_checksum = if info.valid_checksum.is_ok() { 0 } else { 1 };
	    let checksum_error_info = match info.valid_checksum {
		Ok(_) => vinparser_checksum_error_info_t { expected: 00, received: 00 },
		Err(err) => vinparser_checksum_error_info_t { expected: err.expected as i8, received: err.received as i8 }
	    };
	    *vinparser_vin = vinparser_vin_t {
		vin: info_vin,
		country: info_country,
		manufacturer: info_manufacturer,
		region: info_region,
		valid_checksum: info_valid_checksum,
		checksum_error_info: checksum_error_info
	    };
	    0
	},
	Err(err) => match err {
	    VINError::IncorrectLength => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_INCORRECTLENGTH;
		-1
	    },
	    VINError::InvalidCharacters(_) => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_INVALIDCHARACTERS;
		-1
	    },
	    VINError::ChecksumError(_) => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_CHECKSUMERROR;
		-1
	    }
	}
    }
}

#[no_mangle]
unsafe extern "C" fn vinparser_verify_checksum(vin: *const c_char, error_code: *mut vinparser_error_t) -> c_int {
    let vin_rs = match CStr::from_ptr(vin).to_str() {
	Ok(str) => str,
	Err(_) => return -1
    };
    match verify_checksum(vin_rs) {
	Ok(_) => 0,
	Err(err) => match err {
	    VINError::IncorrectLength => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_INCORRECTLENGTH;
		-1
	    },
	    VINError::InvalidCharacters(_) => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_INVALIDCHARACTERS;
		-1
	    },
	    VINError::ChecksumError(_) => {
		*error_code = vinparser_error_t::VINPARSER_ERROR_CHECKSUMERROR;
		-1
	    }
	}
    }
}

// smbc is library wrapping libsmbclient from Samba project
// Copyright (c) 2016 Konstantin Gribov
//
// This file is part of smbc.
//
// smbc is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// smbc is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with smbc. If not, see <http://www.gnu.org/licenses/>.

use libc::{c_char, c_int};

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::slice;

use result::*;

#[inline(always)]
/// Ok(ptr) for non-null ptr or Err(last_os_error) otherwise
pub fn result_from_ptr_mut<T>(ptr: *mut T) -> io::Result<*mut T> {
    if ptr.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(ptr)
    }
}

#[inline(always)]
/// Ok(ptr) for non-null ptr or Err(last_os_error) otherwise
pub fn result_from_ptr<T>(ptr: *const T) -> io::Result<*const T> {
    if ptr.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(ptr)
    }
}

pub unsafe fn cstr<'a, T>(p: *const T) -> Cow<'a, str> {
    CStr::from_ptr(p as *const c_char).to_string_lossy()
}

pub fn cstring<P: AsRef<str>>(p: P) -> Result<CString> {
    Ok(CString::new(p.as_ref())?)
}

pub unsafe fn write_to_cstr(dest: *mut u8, len: usize, src: &str) {
    // just to ensure that it can be interpreted as c string
    *(dest.offset((len - 1) as isize)) = 0u8;
    trace!(target: "smbc", "orig: {:?}", cstr(dest));

    let mut buf = slice::from_raw_parts_mut(dest, len);
    let mut idx = buf.write(src.as_bytes()).unwrap();

    if idx == len {
        idx -= 1;
    }
    buf = slice::from_raw_parts_mut(dest, len);
    buf[idx] = 0u8;

    trace!(target: "smbc", "write to [{:p};{}] from [{:p},{}]: {:?}", dest, len, src.as_ptr(), src.len(), cstr(dest));
}

#[inline(always)]
/// to io::Result with Err(last_os_error) if t == -1
pub fn to_result_with_le<T: Eq + From<i8>>(t: T) -> io::Result<T> {
    to_result_with_error(t, io::Error::last_os_error())
}

#[inline(always)]
/// to io::Result with Err(from_raw_os_error(errno)) if t == -1
pub fn to_result_with_errno<T: Eq + From<i8>>(t: T, errno: c_int) -> io::Result<T> {
    to_result_with_error(t, io::Error::from_raw_os_error(errno as i32))
}

#[inline(always)]
fn to_result_with_error<T: Eq + From<i8>>(t: T, err: io::Error) -> io::Result<T> {
    if t == T::from(-1) {
        Err(err)
    } else {
        Ok(t)
    }
}

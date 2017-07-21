// This file is part of pointer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT. No part of pointer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of pointer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT.


#![allow(non_upper_case_globals)] 
#![allow(non_snake_case)]


extern crate rust_extra;


use ::rust_extra::unlikely;
use ::rust_extra::likely;
use ::std::mem::size_of;
use ::std::ptr::read;
use ::std::ptr::copy;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::swap;
use ::std::ptr::write;
use ::std::ptr::write_bytes;


include!("CopyPointer.rs");
include!("Memory.rs");
include!("MutableMemory.rs");
include!("MutablePointer.rs");
include!("MutableTPointer.rs");
include!("Pointer.rs");
include!("SizedPointer.rs");
include!("TPointer.rs");

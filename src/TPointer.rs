// This file is part of pointer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT. No part of pointer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of pointer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT.


pub trait TPointer<T>
{
	#[inline(always)]
    fn read(self) -> T;
	
	// AKA memmove
	#[inline(always)]
    fn copyAssumingOverlap(self, destination: *mut T, count: usize);
	
	// AKA memcpy
	#[inline(always)]
	fn copyAssumingNoOverlap(self, destination: *mut T, count: usize);
}

impl<T> TPointer<T> for *const T
{
	#[inline(always)]
    fn read(self) -> T
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe { read(self) }
	}

	#[inline(always)]
    fn copyAssumingOverlap(self, destination: *mut T, count: usize)
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		debug_assert!(!destination.is_null(), "destination is a null pointer");
		
		unsafe { copy(self, destination, count) }
	}

	#[inline(always)]
	fn copyAssumingNoOverlap(self, destination: *mut T, count: usize)
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe { copy_nonoverlapping(self, destination, count) }
	}
}

impl<T> TPointer<T> for *mut T
{
	#[inline(always)]
    fn read(self) -> T
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe { read(self) }
	}

	#[inline(always)]
    fn copyAssumingOverlap(self, destination: *mut T, count: usize)
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		debug_assert!(!destination.is_null(), "destination is a null pointer");
		
		unsafe { copy(self, destination, count) }
	}

	#[inline(always)]
	fn copyAssumingNoOverlap(self, destination: *mut T, count: usize)
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		debug_assert!(!destination.is_null(), "destination is a null pointer");
		
		unsafe { copy_nonoverlapping(self, destination, count) }
	}
}

// This file is part of pointer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT. No part of pointer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of pointer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT.


pub trait MutableTPointer<T>
{
	#[inline(always)]
	fn write(self, source: T);

	#[inline(always)]
	fn swap(self, source: *mut T);
}

impl<T> MutableTPointer<T> for *mut T
{
	#[inline(always)]
	fn write(self, source: T)
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe { write(self, source) }
	}

	#[inline(always)]
	fn swap(self, source: *mut T)
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		debug_assert!(!source.is_null(), "destination is a null pointer");
		
		unsafe { swap(self, source) };
	}
}

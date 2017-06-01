// This file is part of pointer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT. No part of pointer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of pointer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT.


pub trait Pointer
{
	#[cfg(target_pointer_width = "16")]
	const WidthInBytes: usize = 2;
	
	#[cfg(target_pointer_width = "32")]
	const WidthInBytes: usize = 4;
	
	#[cfg(target_pointer_width = "64")]
	const WidthInBytes: usize = 8;
	
	#[cfg(target_pointer_width = "16")]
	const WidthInBits: usize = 16;
	
	#[cfg(target_pointer_width = "32")]
	const WidthInBits: usize = 32;
	
	#[cfg(target_pointer_width = "64")]
	const WidthInBits: usize = 64;
	
	#[inline(always)]
	fn offsetUp(self, offset: usize) -> Self;
	
	#[inline(always)]
	fn offsetDown(self, offset: usize) -> Self;
	
	/// Note, at runtime, this code can fail if the difference is greater than isize::MAX
	/// In practice, this is highly unlikely
	#[inline(always)]
	fn numberOfElements(self, higher: Self) -> isize;
}

macro_rules! numberOfElements
{
	($self: ident, $higher: ident) =>
	{
		{
			debug_assert!(!$self.is_null(), "self is a null pointer");
			debug_assert!(!$higher.is_null(), "higher is a null pointer");
		
			let higher = $higher as usize;
			let lower = $self as usize;
			if unlikely(higher == lower)
			{
				0
			}
			else
			{
				let difference = (if likely(higher > lower)
				{
					higher - lower
				}
				else
				{
					lower - higher
				}) as isize;

				let sizeOfOurself = size_of::<T>();
				if sizeOfOurself == 0
				{
					difference
				}
				else
				{
					difference / (sizeOfOurself as isize)
				}
			}
		}
	}
}

impl<T> Pointer for *const T
{
	#[inline(always)]
	fn offsetUp(self, offset: usize) -> Self
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe
		{
			self.offset(offset as isize)
		}
	}
	
	#[inline(always)]
	fn offsetDown(self, offset: usize) -> Self
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe
		{
			self.offset(-(offset as isize))
		}
	}
	
	#[inline(always)]
	fn numberOfElements(self, higher: Self) -> isize
	{
		numberOfElements!(self, higher)
	}
}

impl<T> Pointer for *mut T
{
	#[inline(always)]
	fn offsetUp(self, offset: usize) -> Self
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe
		{
			self.offset(offset as isize)
		}
	}
	
	#[inline(always)]
	fn offsetDown(self, offset: usize) -> Self
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe
		{
			self.offset(-(offset as isize))
		}
	}
	
	#[inline(always)]
	fn numberOfElements(self, higher: Self) -> isize
	{
		numberOfElements!(self, higher)
	}
}

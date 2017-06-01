// This file is part of pointer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT. No part of pointer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of pointer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT.


pub trait CopyPointer: Pointer + Copy
{
	#[inline(always)]
	fn increment(&mut self)
	{
		*self = self.offsetUp(1);
	}
	
	#[inline(always)]
	fn decrement(&mut self)
	{
		*self = self.offsetDown(1);
	}
	
	/// ++self
	#[inline(always)]
	fn preIncrement(&mut self) -> Self
	{
		*self = self.offsetUp(1);
		*self
	}
	
	/// self++
	#[inline(always)]
	fn postIncrement(&mut self) -> Self
	{
		let value = *self;
		*self = self.offsetUp(1);
		value
	}
	
	/// --self
	#[inline(always)]
	fn preDecrement(&mut self) -> Self
	{
		*self = self.offsetDown(1);
		*self
	}
	
	/// self--
	#[inline(always)]
	fn postDecrement(&mut self) -> Self
	{
		let value = *self;
		*self = self.offsetDown(1);
		value
	}
}

impl<T> CopyPointer for *const T
{
}

impl<T> CopyPointer for *mut T
{
}

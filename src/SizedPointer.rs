// This file is part of pointer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT. No part of pointer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of pointer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT.


pub trait SizedPointer: Pointer + Sized
{
	/// Eg if an array consists of 4 elements of size 11, to get to the start of the 2nd element use strideSize=11, index=1
	#[inline(always)]
	fn strideIncrement(self, strideSize: usize, index: usize) -> Self
	{
		self.offsetUp(strideSize * index)
	}
	
	#[inline(always)]
	fn strideDecrement(self, strideSize: usize, index: usize) -> Self
	{
		self.offsetDown(strideSize * index)
	}
}

impl<T> SizedPointer for *const T
{
}

impl<T> SizedPointer for *mut T
{
}

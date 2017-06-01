// This file is part of pointer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT. No part of pointer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of pointer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/pointer/master/COPYRIGHT.


pub trait MutableMemory
{
	/// This tries to memset(pointer, offset), even if *const!
	/// Sadly, many, many compilers optimise the equivalent out in C (explicit_bzero) and Rust is no better in this regard
	/// Eventually, it is to be hoped that clang or gcc or Rust provide an intrinsic for this
	#[inline(always)]
	fn explicit_bzero(self, offset: usize);

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeValue<T>(self, value: T) -> usize;

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeRef<T>(self, value: &T) -> usize;

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeConst<T>(self, value: *const T) -> usize;
	
	#[inline(always)]
	fn writeByte(self, value: u8) -> usize;
	
	#[inline(always)]
	fn writeBytes(self, source: *const u8, count: usize) -> usize;
	
	#[inline(always)]
	fn writeU16AsNetworkByteOrderU16(self, size: u16) -> usize;
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU8(self, size: usize) -> usize;
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU16(self, size: usize) -> usize;
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU24(self, size: usize) -> usize;
}

impl MutableMemory for *mut u8
{
	#[inline(always)]
	fn explicit_bzero(self, offset: usize)
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe
		{
			write_bytes(self, 0, offset);
		}
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeValue<T>(self, value: T) -> usize
	{
		self.writeRef(&value)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeRef<T>(self, value: &T) -> usize
	{
		self.writeConst(value as *const _)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeConst<T>(self, value: *const T) -> usize
	{
		self.writeBytes(value as *const u8, size_of::<T>())
	}
	
	#[inline(always)]
	fn writeByte(self, value: u8) -> usize
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		
		unsafe
		{
			write(self, value);
		}
		
		1
	}
	
	#[inline(always)]
	fn writeBytes(self, source: *const u8, count: usize) -> usize
	{
		debug_assert!(!self.is_null(), "self is a null pointer");
		debug_assert!(!source.is_null(), "source is a null pointer");
		
		unsafe
		{
			copy_nonoverlapping(source, self, count);
		}
		
		count
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeU16AsNetworkByteOrderU16(self, size: u16) -> usize
	{
		let pointer = &size as *const u16 as *const u8;
		
		#[cfg(target_endian = "little")]
		unsafe
		{
			self.writeByte(*pointer.offsetUp(1));
			self.writeByte(*pointer);
		}
	
		#[cfg(target_endian = "big")]
		unsafe
		{
			self.writeBytes(*pointer, 2);
		}
		
		2
	}
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU8(self, size: usize) -> usize
	{
		debug_assert!(size < 256, "size is equal to or more than {}", 256);
	
		self.writeByte(size as u8)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU16(self, size: usize) -> usize
	{
		debug_assert!(size < 65536, "size is equal to or more than {}", 65536);
	
		let pointer = &size as *const usize as *const u8;
	
		#[cfg(target_endian = "little")]
		unsafe
		{
			self.writeByte(*pointer.offsetUp(1));
			self.writeByte(*pointer);
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 32))]
		unsafe
		{
			self.writeBytes(*pointer.offsetUp(2), 2);
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 64))]
		unsafe
		{
			self.writeBytes(*pointer.offsetUp(6), 2);
		}
		
		2
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU24(self, size: usize) -> usize
	{
		debug_assert!(size < 16777216, "size is equal to or more than {}", 16777216);
	
		let pointer = &size as *const usize as *const u8;
	
		#[cfg(target_endian = "little")]
		unsafe
		{
			self.writeByte(*pointer.offsetUp(2));
			self.writeByte(*pointer.offsetUp(1));
			self.writeByte(*pointer);
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 32))]
		unsafe
		{
			self.writeBytes(*pointer.offsetUp(1), 3);
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 64))]
		unsafe
		{
			self.writeBytes(*pointer.offsetUp(5), 3);
		}
		
		3
	}
}

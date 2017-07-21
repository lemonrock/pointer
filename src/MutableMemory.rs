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
	fn writeU16HostEndianOrder(self, value: u16) -> usize;
	
	#[inline(always)]
	fn writeU32HostEndianOrder(self, value: u32) -> usize;
	
	#[inline(always)]
	fn writeU64HostEndianOrder(self, value: u64) -> usize;
	
	#[inline(always)]
	fn writeI8HostEndianOrder(self, value: i8) -> usize;
	
	#[inline(always)]
	fn writeI16HostEndianOrder(self, value: i16) -> usize;
	
	#[inline(always)]
	fn writeI32HostEndianOrder(self, value: i32) -> usize;
	
	#[inline(always)]
	fn writeI64HostEndianOrder(self, value: i64) -> usize;
	
	#[inline(always)]
	fn writeU16AsNetworkByteOrderU16(self, value: u16) -> usize;
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU8(self, value: usize) -> usize;
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU16(self, value: usize) -> usize;
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU24(self, value: usize) -> usize;
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
	
	#[inline(always)]
	fn writeU16HostEndianOrder(self, value: u16) -> usize
	{
		let pointer = &value as *const u16 as *const u8;

		self.writeBytes(pointer, 2)
	}
	
	#[inline(always)]
	fn writeU32HostEndianOrder(self, value: u32) -> usize
	{
		let pointer = &value as *const u32 as *const u8;

		self.writeBytes(pointer, 4)
	}
	
	#[inline(always)]
	fn writeU64HostEndianOrder(self, value: u64) -> usize
	{
		let pointer = &value as *const u64 as *const u8;

		self.writeBytes(pointer, 8)
	}
	
	#[inline(always)]
	fn writeI8HostEndianOrder(self, value: i8) -> usize
	{
		let pointer = &value as *const i8 as *const u8;

		self.writeBytes(pointer, 1)
	}
	
	#[inline(always)]
	fn writeI16HostEndianOrder(self, value: i16) -> usize
	{
		let pointer = &value as *const i16 as *const u8;

		self.writeBytes(pointer, 2)
	}
	
	#[inline(always)]
	fn writeI32HostEndianOrder(self, value: i32) -> usize
	{
		let pointer = &value as *const i32 as *const u8;

		self.writeBytes(pointer, 4)
	}
	
	#[inline(always)]
	fn writeI64HostEndianOrder(self, value: i64) -> usize
	{
		let pointer = &value as *const i64 as *const u8;

		self.writeBytes(pointer, 8)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeU16AsNetworkByteOrderU16(self, value: u16) -> usize
	{
		let pointer = &value as *const u16 as *const u8;
		
		#[cfg(target_endian = "little")]
		unsafe
		{
			self.writeByte(*pointer.offsetUp(1));
			self.writeByte(*pointer);
		}
	
		#[cfg(target_endian = "big")]
		{
			self.writeBytes(pointer, 2);
		}
		
		2
	}
	
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU8(self, value: usize) -> usize
	{
		debug_assert!(value < 256, "size is equal to or more than {}", 256);
	
		self.writeByte(value as u8)
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU16(self, value: usize) -> usize
	{
		debug_assert!(value < 65536, "size is equal to or more than {}", 65536);
	
		let pointer = &value as *const usize as *const u8;
	
		#[cfg(target_endian = "little")]
		unsafe
		{
			self.writeByte(*pointer.offsetUp(1));
			self.writeByte(*pointer);
			return 2;
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 32))]
		{
			return self.writeBytes(pointer.offsetUp(2), 2);
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 64))]
		{
			return self.writeBytes(pointer.offsetUp(6), 2);
		}
	}

	#[allow(trivial_casts)]
	#[inline(always)]
	fn writeUsizeAsNetworkByteOrderU24(self, value: usize) -> usize
	{
		debug_assert!(value < 16777216, "size is equal to or more than {}", 16777216);
	
		let pointer = &value as *const usize as *const u8;
	
		#[cfg(target_endian = "little")]
		unsafe
		{
			self.writeByte(*pointer.offsetUp(2));
			self.writeByte(*pointer.offsetUp(1));
			self.writeByte(*pointer);
			return 3;
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 32))]
		{
			return self.writeBytes(pointer.offsetUp(1), 3);
		}
	
		#[cfg(all(target_endian = "big", target_pointer_width = 64))]
		{
			return self.writeBytes(pointer.offsetUp(5), 3);
		}
	}
}

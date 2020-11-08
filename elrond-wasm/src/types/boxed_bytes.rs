use alloc::boxed::Box;
use elrond_codec::*;

/// Simple wrapper around a boxed byte slice,
/// but with a lot of optimized methods for manipulating it.
/// The focus is on readucing code size rather improving speed.
pub struct BoxedBytes(Box<[u8]>);

impl BoxedBytes {
	pub fn empty() -> Self {
		BoxedBytes(Box::from([0u8; 0]))
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}
}

impl AsRef<[u8]> for BoxedBytes {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		&*self.0
	}
}

impl<'a> From<&'a [u8]> for BoxedBytes {
	#[inline]
	fn from(byte_slice: &'a [u8]) -> Self {
		BoxedBytes(Box::from(byte_slice))
	}
}

impl NestedEncode for BoxedBytes {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		dest.write(self.as_ref());
		Ok(())
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		_: ExitCtx,
		_: fn(ExitCtx, EncodeError) -> !,
	) {
		dest.write(self.as_ref());
	}
}

impl TopEncode for BoxedBytes {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		output.set_slice_u8(self.as_ref());
		Ok(())
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		_: ExitCtx,
		_: fn(ExitCtx, EncodeError) -> !,
	) {
		output.set_slice_u8(self.as_ref());
	}
}

impl NestedDecode for BoxedBytes {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let size = usize::dep_decode(input)?;
		let byte_slice = input.read_slice(size)?;
		Ok(byte_slice.into())
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let size = usize::dep_decode_or_exit(input, c.clone(), exit);
		let byte_slice = input.read_slice_or_exit(size, c, exit);
		byte_slice.into()
	}
}

impl TopDecode for BoxedBytes {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(BoxedBytes(input.into_boxed_slice_u8()))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		BoxedBytes(input.into_boxed_slice_u8())
	}
}
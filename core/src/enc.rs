/*
Code mostly inspired by bincode, modified to create binary encoding for the Hive Blockchain.

--- BEGIN ORIGINAL LICENSE ---
The MIT License (MIT)

Copyright (c) 2014 Ty Overby

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
--- END ORIGINAL LICENSE ---
*/
use chrono::{DateTime, Utc};

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError>;
}

impl<T: Writer> Writer for &mut T {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        (**self).write(bytes)
    }
}

#[derive(Debug)]
pub enum EncodeError {
    Error,
}

pub trait HiveEncode {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError>;
}

pub trait HiveEncoder {
    type W: Writer;

    fn writer(&mut self) -> &mut Self::W;
}

impl<'a, T> HiveEncoder for &'a mut T
where
    T: HiveEncoder,
{
    type W = T::W;
    fn writer(&mut self) -> &mut Self::W {
        T::writer(self)
    }
}

impl HiveEncode for () {
    fn encode<E: HiveEncoder>(&self, _encoder: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl HiveEncode for bool {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if *self { 1u8 } else { 0u8 }.encode(encoder)
    }
}

macro_rules! primitive_impl {
    ( $t:ty ) => {
        impl HiveEncode for $t {
            fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
                encoder.writer().write(&self.to_le_bytes())
            }
        }
    };
}

primitive_impl!(u8);
primitive_impl!(u16);
primitive_impl!(u32);
primitive_impl!(u64);
primitive_impl!(u128);
primitive_impl!(usize);
primitive_impl!(i16);
primitive_impl!(i32);
primitive_impl!(i64);
primitive_impl!(i128);
primitive_impl!(isize);
primitive_impl!(f32);
primitive_impl!(f64);

#[inline]
fn encode_leb128_unsigned<E: HiveEncoder>(encoder: &mut E, v: u64) -> Result<(), EncodeError> {
    let mut buf = [0; 128];
    let mut writable = &mut buf[..];
    // Array lengths needs to be written als LEB128
    let n = leb128::write::unsigned(&mut writable, v).map_err(|_| EncodeError::Error)?;

    encoder.writer().write(&buf[..n])
}

#[inline]
fn encode_slice_len<E: HiveEncoder>(encoder: &mut E, len: usize) -> Result<(), EncodeError> {
    encode_leb128_unsigned(encoder, len as u64)
}

impl<T> HiveEncode for [T]
where
    T: HiveEncode,
{
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_slice_len(encoder, self.len())?;
        for item in self {
            item.encode(encoder)?;
        }

        Ok(())
    }
}

impl HiveEncode for str {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

impl<T, const N: usize> HiveEncode for [T; N]
where
    T: HiveEncode,
{
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_slice_len(encoder, N)?;

        for item in self.iter() {
            item.encode(encoder)?;
        }

        Ok(())
    }
}

impl<'a, T> HiveEncode for &'a T
where
    T: HiveEncode + ?Sized,
{
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

impl HiveEncode for char {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_utf8(encoder.writer(), *self)
    }
}

fn encode_utf8(writer: &mut impl Writer, c: char) -> Result<(), EncodeError> {
    let mut buf = vec![0u8; c.len_utf8()];
    c.encode_utf8(buf.as_mut());

    writer.write(&buf)
}

enum CastError {
    Overflow,
    Underflow,
}

fn cast_i64_to_u32(src: i64) -> Result<u32, CastError> {
    Err(if src < u32::MIN as i64 {
        CastError::Underflow
    } else if src > u32::MAX as i64 {
        CastError::Overflow
    } else {
        return Ok(src as u32);
    })
}

impl HiveEncode for DateTime<Utc> {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        HiveEncode::encode(
            &cast_i64_to_u32(self.timestamp()).map_err(|_| EncodeError::Error)?,
            encoder,
        )
    }
}

pub struct LEB128(u64);

impl From<u8> for LEB128 {
    fn from(from: u8) -> Self {
        LEB128(from as u64)
    }
}

impl From<u16> for LEB128 {
    fn from(from: u16) -> Self {
        LEB128(from as u64)
    }
}

impl From<u32> for LEB128 {
    fn from(from: u32) -> Self {
        LEB128(from as u64)
    }
}

impl From<u64> for LEB128 {
    fn from(from: u64) -> Self {
        LEB128(from)
    }
}

impl HiveEncode for LEB128 {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_leb128_unsigned(encoder, self.0)
    }
}

impl<T> HiveEncode for Vec<T>
where
    T: HiveEncode,
{
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_slice_len(encoder, self.len())?;

        for item in self.iter() {
            item.encode(encoder)?;
        }

        Ok(())
    }
}

impl HiveEncode for String {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

#[derive(Default)]
struct VecWriter {
    inner: Vec<u8>,
}

impl VecWriter {
    fn collect(self) -> Vec<u8> {
        self.inner
    }
}

impl Writer for VecWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.inner.extend_from_slice(bytes);
        Ok(())
    }
}

struct HiveEncoderImpl<W: Writer> {
    writer: W,
}

impl<W: Writer> HiveEncoderImpl<W> {
    pub fn new(writer: W) -> HiveEncoderImpl<W> {
        HiveEncoderImpl { writer }
    }

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<W: Writer> HiveEncoder for HiveEncoderImpl<W> {
    type W = W;

    fn writer(&mut self) -> &mut Self::W {
        &mut self.writer
    }
}

pub fn encode_to_vec<T>(t: T) -> Result<Vec<u8>, EncodeError>
where
    T: HiveEncode,
{
    let mut encoder = HiveEncoderImpl::new(VecWriter::default());
    T::encode(&t, &mut encoder)?;

    Ok(encoder.into_writer().collect())
}

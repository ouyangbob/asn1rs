use crate::syn::{ReadableType, Reader, WritableType, Writer};
use core::marker::PhantomData;

pub struct OpenType<C: Constraint>(PhantomData<C>);

pub trait Constraint: super::common::Constraint + Sized {
    const NAME: &'static str;
    const VARIANT_COUNT: u64;
    const STD_VARIANT_COUNT: u64;
    const EXTENSIBLE: bool = false;

    fn to_choice_index(&self) -> usize;

    fn write_content<W: Writer>(&self, writer: &mut W) -> Result<(), W::Error>;

    fn read_content<R: Reader>(index: usize, reader: &mut R) -> Result<Option<Self>, R::Error>;
}

impl<C: Constraint> WritableType for OpenType<C> {
    type Type = C;

    #[inline]
    fn write_value<W: Writer>(
        writer: &mut W,
        value: &Self::Type,
    ) -> Result<(), <W as Writer>::Error> {
        writer.write_open_type(value)
    }
}

impl<C: Constraint> ReadableType for OpenType<C> {
    type Type = C;

    fn read_value<R: Reader>(_reader: &mut R) -> Result<Self::Type, <R as Reader>::Error> {
        panic!("not support----OpenType&ReadableType")
    }

    #[inline]
    fn read_value_by_key<R: Reader>(reader: &mut R, key: usize) -> Result<Self::Type, <R as Reader>::Error> {
        reader.read_open_type::<Self::Type>(key)
    }
}

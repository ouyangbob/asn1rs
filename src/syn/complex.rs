use crate::syn::{Readable, ReadableType, Reader, Writable, WritableType, Writer};
use core::marker::PhantomData;

pub struct Complex<V, T: Constraint>(PhantomData<T>, PhantomData<V>,Option<usize>);

pub trait Constraint: super::common::Constraint {

}

impl<V: Writable, C: Constraint> WritableType for Complex<V, C> {
    type Type = V;

    #[inline]
    fn write_value<W: Writer>(
        writer: &mut W,
        value: &Self::Type,
    ) -> Result<(), <W as Writer>::Error> {
        value.write(writer)
    }
}

impl<V: Readable, C: Constraint> ReadableType for Complex<V, C> {
    type Type = V;

    fn read_value<R: Reader>(reader: &mut R) -> Result<Self::Type, R::Error>{
        V::read(reader)
    }

    #[inline]
    fn read_value_by_key<R: Reader>(reader: &mut R, key: usize) -> Result<Self::Type, <R as Reader>::Error> {
        V::read_by_key(reader,key)
    }
}

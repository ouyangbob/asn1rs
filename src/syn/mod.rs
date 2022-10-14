pub mod bitstring;
pub mod boolean;
pub mod choice;
pub mod common;
pub mod complex;
pub mod default;
pub mod enumerated;
pub mod ia5string;
pub mod io;
pub mod null;
pub mod numbers;
pub mod numericstring;
pub mod octetstring;
pub mod optional;
pub mod printablestring;
pub mod sequence;
pub mod sequenceof;
pub mod set;
pub mod setof;
pub mod utf8string;
pub mod visiblestring;
pub mod opentype;

pub use crate::syn::null::Null;
pub use bitstring::BitString;
pub use bitstring::BitVec;
pub use boolean::Boolean;
pub use choice::Choice;
pub use opentype::OpenType;
pub use complex::Complex;
pub use default::DefaultValue;
pub use enumerated::Enumerated;
pub use ia5string::Ia5String;
pub use null::NullT;
pub use numbers::Integer;
pub use numericstring::NumericString;
pub use octetstring::OctetString;
pub use printablestring::PrintableString;
pub use sequence::Sequence;
pub use sequenceof::SequenceOf;
pub use set::Set;
pub use setof::SetOf;
pub use utf8string::Utf8String;
pub use visiblestring::VisibleString;

pub mod prelude {
    pub use super::bitstring::BitVec;
    pub use super::Null;
    pub use super::Readable;
    pub use super::ReadableType;
    pub use super::Reader;
    pub use super::Writable;
    pub use super::WritableType;
    pub use super::Writer;
}

pub trait Reader {
    type Error;

    #[inline]
    fn read<T: Readable>(&mut self) -> Result<T, Self::Error>
    where
        Self: Sized,
    {
        T::read(self)
    }

    fn read_sequence<
        C: sequence::Constraint,
        S: Sized,
        F: Fn(&mut Self) -> Result<S, Self::Error>,
    >(
        &mut self,
        f: F,
    ) -> Result<S, Self::Error>;

    fn read_sequence_of<C: sequenceof::Constraint, T: ReadableType>(
        &mut self,
    ) -> Result<Vec<T::Type>, Self::Error>;

    fn read_set<C: set::Constraint, S: Sized, F: Fn(&mut Self) -> Result<S, Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<S, Self::Error>;

    fn read_set_of<C: setof::Constraint, T: ReadableType>(
        &mut self,
    ) -> Result<Vec<T::Type>, Self::Error>;

    fn read_enumerated<C: enumerated::Constraint>(&mut self) -> Result<C, Self::Error>;

    fn read_choice<C: choice::Constraint>(&mut self) -> Result<C, Self::Error>;
    fn read_open_type<C: opentype::Constraint>(&mut self,key:usize) -> Result<C, Self::Error>;

    fn read_opt<T: ReadableType>(&mut self) -> Result<Option<T::Type>, Self::Error>;

    fn read_default<C: default::Constraint<Owned = T::Type>, T: ReadableType>(
        &mut self,
    ) -> Result<T::Type, Self::Error>;

    fn read_number<T: numbers::Number, C: numbers::Constraint<T>>(
        &mut self,
    ) -> Result<T, Self::Error>;

    fn read_utf8string<C: utf8string::Constraint>(&mut self) -> Result<String, Self::Error>;

    fn read_ia5string<C: ia5string::Constraint>(&mut self) -> Result<String, Self::Error>;

    fn read_numeric_string<C: numericstring::Constraint>(&mut self) -> Result<String, Self::Error>;

    fn read_visible_string<C: visiblestring::Constraint>(&mut self) -> Result<String, Self::Error>;

    fn read_printable_string<C: printablestring::Constraint>(
        &mut self,
    ) -> Result<String, Self::Error>;

    fn read_octet_string<C: octetstring::Constraint>(&mut self) -> Result<Vec<u8>, Self::Error>;

    fn read_bit_string<C: bitstring::Constraint>(&mut self) -> Result<(Vec<u8>, u64), Self::Error>;

    fn read_boolean<C: boolean::Constraint>(&mut self) -> Result<bool, Self::Error>;

    fn read_null<C: null::Constraint>(&mut self) -> Result<Null, Self::Error>;
}

pub trait Readable: Sized {
    fn read<R: Reader>(reader: &mut R) -> Result<Self, R::Error>;

    fn read_by_key<R: Reader>(_reader: &mut R,_key:usize) -> Result<Self, R::Error>{
        panic!("not support----Readable&trait&root")
    }
}

pub trait ReadableType {
    type Type: Sized;

    fn read_value<R: Reader>(reader: &mut R) -> Result<Self::Type, R::Error>;
    fn read_value_by_key<R: Reader>(_reader: &mut R,_key:usize) -> Result<Self::Type, R::Error>{
        panic!("not support----ReadableType&trait&root")
    }
}

impl<T: Readable> ReadableType for T {
    type Type = T;

    #[inline]
    fn read_value<R: Reader>(reader: &mut R) -> Result<T, R::Error> {
        T::read(reader)
    }
}

pub trait Writer {
    type Error;

    #[inline]
    fn write<T: Writable>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        value.write(self)
    }

    fn write_sequence<C: sequence::Constraint, F: Fn(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), Self::Error>;

    fn write_sequence_of<C: sequenceof::Constraint, T: WritableType>(
        &mut self,
        slice: &[T::Type],
    ) -> Result<(), Self::Error>;

    fn write_set<C: set::Constraint, F: Fn(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), Self::Error>;

    fn write_set_of<C: setof::Constraint, T: WritableType>(
        &mut self,
        slice: &[T::Type],
    ) -> Result<(), Self::Error>;

    fn write_enumerated<C: enumerated::Constraint>(
        &mut self,
        enumerated: &C,
    ) -> Result<(), Self::Error>;

    fn write_choice<C: choice::Constraint>(&mut self, choice: &C) -> Result<(), Self::Error>;
    fn write_open_type<C: opentype::Constraint>(&mut self, opentype: &C) -> Result<(), Self::Error>;

    fn write_opt<T: WritableType>(&mut self, value: Option<&T::Type>) -> Result<(), Self::Error>;

    fn write_default<C: default::Constraint<Owned = T::Type>, T: WritableType>(
        &mut self,
        value: &T::Type,
    ) -> Result<(), Self::Error>;

    fn write_number<T: numbers::Number, C: numbers::Constraint<T>>(
        &mut self,
        value: T,
    ) -> Result<(), Self::Error>;

    fn write_utf8string<C: utf8string::Constraint>(
        &mut self,
        value: &str,
    ) -> Result<(), Self::Error>;

    fn write_ia5string<C: ia5string::Constraint>(&mut self, value: &str)
        -> Result<(), Self::Error>;

    fn write_numeric_string<C: numericstring::Constraint>(
        &mut self,
        value: &str,
    ) -> Result<(), Self::Error>;

    fn write_visible_string<C: visiblestring::Constraint>(
        &mut self,
        value: &str,
    ) -> Result<(), Self::Error>;

    fn write_printable_string<C: printablestring::Constraint>(
        &mut self,
        value: &str,
    ) -> Result<(), Self::Error>;

    fn write_octet_string<C: octetstring::Constraint>(
        &mut self,
        value: &[u8],
    ) -> Result<(), Self::Error>;

    fn write_bit_string<C: bitstring::Constraint>(
        &mut self,
        value: &[u8],
        bit_len: u64,
    ) -> Result<(), Self::Error>;

    fn write_boolean<C: boolean::Constraint>(&mut self, value: bool) -> Result<(), Self::Error>;

    fn write_null<C: null::Constraint>(&mut self, value: &Null) -> Result<(), Self::Error>;
}

pub trait Writable {
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), W::Error>;
}

pub trait WritableType {
    type Type;

    fn write_value<W: Writer>(writer: &mut W, value: &Self::Type) -> Result<(), W::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{ProtobufWriter, UperWriter};
    use crate::syn::common;
    use crate::syn::io::PrintlnWriter;
    use crate::syn::sequence::Sequence;
    use crate::syn::utf8string::Utf8String;
    use asn1rs_model::model::Tag;

    #[test]
    fn test_compilable() {
        #[derive(Debug, PartialEq)]
        struct Whatever {
            name: String,
            opt: Option<String>,
            some: Option<String>,
        }

        type AsnDefWhatever = Sequence<Whatever>;
        type AsnDefWhateverName = Utf8String;
        type AsnDefWhateverOpt = Option<Utf8String>;
        type AsnDefWhateverSome = Option<Utf8String>;

        impl common::Constraint for Whatever {
            const TAG: Tag = Tag::DEFAULT_SEQUENCE;
        }
        impl sequence::Constraint for Whatever {
            const NAME: &'static str = "Whatever";
            const STD_OPTIONAL_FIELDS: u64 = 2;
            const FIELD_COUNT: u64 = 3;
            const EXTENDED_AFTER_FIELD: Option<u64> = None;

            fn read_seq<R: Reader>(reader: &mut R) -> Result<Self, <R as Reader>::Error>
            where
                Self: Sized,
            {
                Ok(Self {
                    name: AsnDefWhateverName::read_value(reader)?,
                    opt: AsnDefWhateverOpt::read_value(reader)?,
                    some: AsnDefWhateverSome::read_value(reader)?,
                })
            }

            fn write_seq<W: Writer>(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
                AsnDefWhateverName::write_value(writer, &self.name)?;
                AsnDefWhateverOpt::write_value(writer, &self.opt)?;
                AsnDefWhateverSome::write_value(writer, &self.some)?;
                Ok(())
            }
        }

        impl Writable for Whatever {
            fn write<W: Writer>(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
                AsnDefWhatever::write_value(writer, self)
            }
        }

        impl Readable for Whatever {
            fn read<R: Reader>(reader: &mut R) -> Result<Self, <R as Reader>::Error> {
                AsnDefWhatever::read_value(reader)
            }
        }

        let mut writer = PrintlnWriter::default();
        let value = Whatever {
            name: "SeGreatName".to_string(),
            opt: None,
            some: Some("Lorem Ipsum".to_string()),
        };

        // Writing sequence Whatever
        //  Writing Utf8String(MIN..MAX): SeGreatName
        //  Writing OPTIONAL
        //   None
        //  Writing OPTIONAL
        //   Some
        //    Writing Utf8String(MIN..MAX): Lorem Ipsum
        //        value.write(&mut writer).unwrap();
        writer.write(&value).unwrap();

        //
        //    Showcase: UPER
        //
        let mut writer = UperWriter::default();
        writer.write(&value).expect("Writing to UPER failed");

        let mut reader = writer.as_reader();
        let read_value = reader.read::<Whatever>().expect("Reading from UPER failed");

        assert_eq!(value, read_value);

        //
        //    Showcase: Protobuf
        //
        let mut writer = ProtobufWriter::default();
        writer.write(&value).expect("Writing to PROTO failed");

        let mut reader = writer.as_reader();
        let read_value = reader
            .read::<Whatever>()
            .expect("Reading from PROTO failed");

        assert_eq!(value, read_value);
    }
}

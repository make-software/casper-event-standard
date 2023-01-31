use alloc::{boxed::Box, vec::Vec};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped,
};

/// A wrapper on top of [`CLType`].
///
/// It is required as the original [`CLType`] doesn't implement [`CLTyped`],
/// [`FromBytes`] and [`ToBytes`].
///
/// It can be removed if this [`issue`] is resolved.
///
/// [`CLType`]: casper_types::CLType
/// [`CLTyped`]: casper_types::CLTyped
/// [`FromBytes`]: casper_types::bytesrepr::FromBytes
/// [`ToBytes`]: casper_types::bytesrepr::ToBytes
/// [`issue`]: https://github.com/casper-network/casper-node/issues/3593
#[derive(PartialEq, Debug)]
pub struct CLType2(pub CLType);

impl CLTyped for CLType2 {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl ToBytes for CLType2 {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result = Vec::new();
        append_bytes(&self.0, &mut result)?;
        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.0.serialized_length()
    }
}

impl FromBytes for CLType2 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        CLType::from_bytes(bytes).map(|(cl_type, bytes)| (CLType2(cl_type), bytes))
    }
}

const CL_TYPE_TAG_BOOL: u8 = 0;
const CL_TYPE_TAG_I32: u8 = 1;
const CL_TYPE_TAG_I64: u8 = 2;
const CL_TYPE_TAG_U8: u8 = 3;
const CL_TYPE_TAG_U32: u8 = 4;
const CL_TYPE_TAG_U64: u8 = 5;
const CL_TYPE_TAG_U128: u8 = 6;
const CL_TYPE_TAG_U256: u8 = 7;
const CL_TYPE_TAG_U512: u8 = 8;
const CL_TYPE_TAG_UNIT: u8 = 9;
const CL_TYPE_TAG_STRING: u8 = 10;
const CL_TYPE_TAG_KEY: u8 = 11;
const CL_TYPE_TAG_UREF: u8 = 12;
const CL_TYPE_TAG_OPTION: u8 = 13;
const CL_TYPE_TAG_LIST: u8 = 14;
const CL_TYPE_TAG_BYTE_ARRAY: u8 = 15;
const CL_TYPE_TAG_RESULT: u8 = 16;
const CL_TYPE_TAG_MAP: u8 = 17;
const CL_TYPE_TAG_TUPLE1: u8 = 18;
const CL_TYPE_TAG_TUPLE2: u8 = 19;
const CL_TYPE_TAG_TUPLE3: u8 = 20;
const CL_TYPE_TAG_ANY: u8 = 21;
const CL_TYPE_TAG_PUBLIC_KEY: u8 = 22;

fn append_bytes(cl_type: &CLType, stream: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
    match cl_type {
        CLType::Bool => stream.push(CL_TYPE_TAG_BOOL),
        CLType::I32 => stream.push(CL_TYPE_TAG_I32),
        CLType::I64 => stream.push(CL_TYPE_TAG_I64),
        CLType::U8 => stream.push(CL_TYPE_TAG_U8),
        CLType::U32 => stream.push(CL_TYPE_TAG_U32),
        CLType::U64 => stream.push(CL_TYPE_TAG_U64),
        CLType::U128 => stream.push(CL_TYPE_TAG_U128),
        CLType::U256 => stream.push(CL_TYPE_TAG_U256),
        CLType::U512 => stream.push(CL_TYPE_TAG_U512),
        CLType::Unit => stream.push(CL_TYPE_TAG_UNIT),
        CLType::String => stream.push(CL_TYPE_TAG_STRING),
        CLType::Key => stream.push(CL_TYPE_TAG_KEY),
        CLType::URef => stream.push(CL_TYPE_TAG_UREF),
        CLType::PublicKey => stream.push(CL_TYPE_TAG_PUBLIC_KEY),
        CLType::Option(cl_type) => {
            stream.push(CL_TYPE_TAG_OPTION);
            append_bytes(cl_type, stream)?;
        }
        CLType::List(cl_type) => {
            stream.push(CL_TYPE_TAG_LIST);
            append_bytes(cl_type, stream)?;
        }
        CLType::ByteArray(len) => {
            stream.push(CL_TYPE_TAG_BYTE_ARRAY);
            stream.append(&mut len.to_bytes()?);
        }
        CLType::Result { ok, err } => {
            stream.push(CL_TYPE_TAG_RESULT);
            append_bytes(ok, stream)?;
            append_bytes(err, stream)?;
        }
        CLType::Map { key, value } => {
            stream.push(CL_TYPE_TAG_MAP);
            append_bytes(key, stream)?;
            append_bytes(value, stream)?;
        }
        CLType::Tuple1(cl_type_array) => {
            serialize_cl_tuple_type(CL_TYPE_TAG_TUPLE1, cl_type_array, stream)?
        }
        CLType::Tuple2(cl_type_array) => {
            serialize_cl_tuple_type(CL_TYPE_TAG_TUPLE2, cl_type_array, stream)?
        }
        CLType::Tuple3(cl_type_array) => {
            serialize_cl_tuple_type(CL_TYPE_TAG_TUPLE3, cl_type_array, stream)?
        }
        CLType::Any => stream.push(CL_TYPE_TAG_ANY),
    }
    Ok(())
}

fn serialize_cl_tuple_type<'a, T: IntoIterator<Item = &'a Box<CLType>>>(
    tag: u8,
    cl_type_array: T,
    stream: &mut Vec<u8>,
) -> Result<(), bytesrepr::Error> {
    stream.push(tag);
    for cl_type in cl_type_array {
        append_bytes(cl_type, stream)?;
    }
    Ok(())
}

// TODO: Tests

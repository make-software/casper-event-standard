use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped,
};

use crate::{cl_type2::CLType2, EventInstance};

/// The information about a single event.
#[derive(Default, Debug, PartialEq)]
pub struct Schema(Vec<(String, CLType2)>);

impl Schema {
    /// Creates an empty object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds new named element.
    pub fn with_elem(&mut self, name: &str, ty: CLType) {
        self.0.push((String::from(name), CLType2(ty)));
    }
}

impl CLTyped for Schema {
    fn cl_type() -> CLType {
        Vec::<(String, CLType2)>::cl_type()
    }
}

impl ToBytes for Schema {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        self.0.to_bytes()
    }

    fn serialized_length(&self) -> usize {
        self.0.serialized_length()
    }
}

impl FromBytes for Schema {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        Vec::<(String, CLType2)>::from_bytes(bytes).map(|(map, bytes)| (Schema(map), bytes))
    }
}

/// The information about multiple events.
#[derive(Default, Debug, PartialEq)]
pub struct Schemas(BTreeMap<String, Schema>);

impl Schemas {
    /// Creates an empty object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds new [`Schema`] based on the event's type.
    pub fn add<T: EventInstance>(&mut self) {
        self.0.insert(T::name(), T::schema());
    }

    /// Adds new [`Schema`] based on the event's type.
    ///
    /// Same as [`add`], but returns Self.
    ///
    /// [`add`]: #method.add
    pub fn with<T: EventInstance>(mut self) -> Self {
        self.add::<T>();
        self
    }
}

impl CLTyped for Schemas {
    fn cl_type() -> CLType {
        BTreeMap::<String, Schema>::cl_type()
    }
}

impl ToBytes for Schemas {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        self.0.to_bytes()
    }

    fn serialized_length(&self) -> usize {
        self.0.serialized_length()
    }
}

impl FromBytes for Schemas {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        BTreeMap::<String, Schema>::from_bytes(bytes).map(|(map, bytes)| (Schemas(map), bytes))
    }
}

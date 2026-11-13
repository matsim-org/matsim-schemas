use std::path::PathBuf;

pub const PROTO_PACKAGE: &str = ".matsim.io.types";
pub const RUST_TYPES_PATH: &str = "::matsim_schemas::matsim::io::types";

const PROTO_FILES: &[&str] = &[
    "matsim/io/types/general.proto",
    "matsim/io/types/events.proto",
    "matsim/io/types/facilities.proto",
    "matsim/io/types/network.proto",
    "matsim/io/types/population.proto",
    "matsim/io/types/transit.proto",
    "matsim/io/types/vehicles.proto",
];

pub fn proto_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto")
}

pub fn proto_files() -> Vec<PathBuf> {
    let root = proto_dir();
    PROTO_FILES.iter().map(|proto| root.join(proto)).collect()
}

#[cfg(feature = "build")]
pub fn configure_extern_path(config: &mut prost_build::Config) -> &mut prost_build::Config {
    config.extern_path(PROTO_PACKAGE, RUST_TYPES_PATH);
    config
}

pub mod matsim {
    pub mod io {
        pub mod types {
            include!(concat!(env!("OUT_DIR"), "/matsim.io.types.rs"));
        }
    }
}

pub mod general {
    pub use crate::matsim::io::types::{AttributeValue, Coordinate, StringMap};
}

pub mod events {
    pub use crate::matsim::io::types::{GenericEvent, TimeStep};
}

pub mod facilities {
    pub use crate::matsim::io::types::{
        ActivityFacilities, ActivityFacility, ActivityOption, OpenDay, OpeningTime,
    };
}

pub mod network {
    pub use crate::matsim::io::types::{Link, Network, Node};
}

pub mod population {
    pub use crate::matsim::io::types::leg::Route;
    pub use crate::matsim::io::types::{
        Activity, GenericRoute, Header, Leg, NetworkRoute, Person, Plan, PtRoute,
        PtRouteDescription,
    };
}

pub mod transit {
    pub use crate::matsim::io::types::{
        MinimalTransferTime, TransitDeparture, TransitLine, TransitRoute, TransitRouteStop,
        TransitSchedule, TransitStopFacility,
    };
}

pub mod vehicles {
    pub use crate::matsim::io::types::{Vehicle, VehicleType, VehiclesContainer};
}

use matsim::io::types::{AttributeValue, StringMap, attribute_value};
use std::collections::HashMap;

impl AttributeValue {
    pub fn new_int(value: i64) -> Self {
        Self {
            r#type: Some(attribute_value::Type::IntValue(value)),
        }
    }

    pub fn new_string(value: impl Into<String>) -> Self {
        Self {
            r#type: Some(attribute_value::Type::StringValue(value.into())),
        }
    }

    pub fn new_double(value: f64) -> Self {
        Self {
            r#type: Some(attribute_value::Type::DoubleValue(value)),
        }
    }

    pub fn new_bool(value: bool) -> Self {
        Self {
            r#type: Some(attribute_value::Type::BoolValue(value)),
        }
    }

    pub fn new_string_map(entries: HashMap<String, String>) -> Self {
        Self {
            r#type: Some(attribute_value::Type::StringMapValue(StringMap { entries })),
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self.r#type {
            Some(attribute_value::Type::IntValue(value)) => Some(value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match &self.r#type {
            Some(attribute_value::Type::StringValue(value)) => Some(value),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self.r#type {
            Some(attribute_value::Type::DoubleValue(value)) => Some(value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self.r#type {
            Some(attribute_value::Type::BoolValue(value)) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_map(&self) -> Option<&HashMap<String, String>> {
        match &self.r#type {
            Some(attribute_value::Type::StringMapValue(map)) => Some(&map.entries),
            _ => None,
        }
    }
}

impl From<i64> for AttributeValue {
    fn from(value: i64) -> Self {
        Self::new_int(value)
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self::new_string(value)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        Self::new_string(value)
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        Self::new_double(value)
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        Self::new_bool(value)
    }
}

impl From<HashMap<String, String>> for AttributeValue {
    fn from(value: HashMap<String, String>) -> Self {
        Self::new_string_map(value)
    }
}

#[cfg(feature = "serde")]
mod serde_attribute_value {
    use super::{AttributeValue, attribute_value};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AttributeValueRepr {
        Bool(bool),
        Int(i64),
        Double(f64),
        String(String),
        StringMap(HashMap<String, String>),
    }

    impl Serialize for AttributeValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match &self.r#type {
                Some(attribute_value::Type::IntValue(value)) => serializer.serialize_i64(*value),
                Some(attribute_value::Type::StringValue(value)) => {
                    serializer.serialize_str(value.as_str())
                }
                Some(attribute_value::Type::DoubleValue(value)) => serializer.serialize_f64(*value),
                Some(attribute_value::Type::BoolValue(value)) => serializer.serialize_bool(*value),
                Some(attribute_value::Type::StringMapValue(map)) => {
                    map.entries.serialize(serializer)
                }
                None => serializer.serialize_none(),
            }
        }
    }

    impl<'de> Deserialize<'de> for AttributeValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(match AttributeValueRepr::deserialize(deserializer)? {
                AttributeValueRepr::Bool(value) => AttributeValue::new_bool(value),
                AttributeValueRepr::Int(value) => AttributeValue::new_int(value),
                AttributeValueRepr::Double(value) => AttributeValue::new_double(value),
                AttributeValueRepr::String(value) => AttributeValue::new_string(value),
                AttributeValueRepr::StringMap(value) => AttributeValue::new_string_map(value),
            })
        }
    }
}

use serde::Deserialize;
use std::str::FromStr;

pub mod bindings;

mod error;

pub use error::Error;

#[cfg(feature = "time")]
mod datetime;

pub use bindings::*;

pub type Result<T> = std::result::Result<T, Error>;

pub trait SnowflakeDeserialize {
    fn snowflake_deserialize(response: SnowflakeSqlResponse) -> Result<SnowflakeSqlResult<Self>>
    where
        Self: Sized;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartitionInfo {
    pub row_count: usize,
    pub uncompressed_size: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SnowflakeSqlResponse {
    pub result_set_meta_data: MetaData,
    pub data: Vec<Vec<Option<String>>>,
    pub code: String,
    pub statement_status_url: String,
    pub request_id: String,
    pub sql_state: String,
    pub message: String,
    pub statement_handle: String,
    //pub created_on: u64,
}

impl SnowflakeSqlResponse {
    pub fn has_partitions(&self) -> bool {
        self.result_set_meta_data.partition_info.len() > 1
    }

    pub fn deserialize<T: SnowflakeDeserialize>(self) -> Result<SnowflakeSqlResult<T>> {
        T::snowflake_deserialize(self)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub num_rows: usize,
    pub format: String,
    pub row_type: Vec<RowType>,
    pub partition_info: Vec<PartitionInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RowType {
    pub name: String,
    pub database: String,
    pub schema: String,
    pub table: String,
    pub precision: Option<u32>,
    pub byte_length: Option<usize>,
    #[serde(rename = "type")]
    pub data_type: String,
    pub scale: Option<i32>,
    pub nullable: bool,
    //pub collation: ???,
    //pub length: ???,
}

#[derive(Debug)]
pub struct SnowflakeSqlResult<T> {
    pub data: Vec<T>,
}

/// For custom data parsing,
/// ex. you want to convert the retrieved data (strings) to enums.
///
/// Data in cells are not their type, they are simply strings that need to be converted.
pub trait DeserializeFromStr {
    fn deserialize_from_str(s: Option<&str>) -> Result<Self>
    where
        Self: Sized;
}

impl<T> DeserializeFromStr for Option<T>
where
    T: DeserializeFromStr,
{
    fn deserialize_from_str(s: Option<&str>) -> Result<Self> {
        if s.is_none() {
            Ok(None)
        } else {
            T::deserialize_from_str(s).map(Some)
        }
    }
}
macro_rules! impl_deserialize_from_str {
    ($ty: ty) => {
        impl DeserializeFromStr for $ty {
            fn deserialize_from_str(s: Option<&str>) -> Result<Self> {
                s.ok_or(Error::UnexpectedNull).and_then(|s| {
                    <$ty>::from_str(s).map_err(|err| Error::Format {
                        given: s.into(),
                        err: err.to_string(),
                    })
                })
            }
        }
    };
}

impl_deserialize_from_str!(bool);
impl_deserialize_from_str!(usize);
impl_deserialize_from_str!(isize);
impl_deserialize_from_str!(u8);
impl_deserialize_from_str!(u16);
impl_deserialize_from_str!(u32);
impl_deserialize_from_str!(u64);
impl_deserialize_from_str!(u128);
impl_deserialize_from_str!(i16);
impl_deserialize_from_str!(i32);
impl_deserialize_from_str!(i64);
impl_deserialize_from_str!(i128);
impl_deserialize_from_str!(f32);
impl_deserialize_from_str!(f64);
impl_deserialize_from_str!(String);
impl_deserialize_from_str!(uuid::Uuid);

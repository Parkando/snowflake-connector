#[derive(Clone, Debug)]
pub enum BindingValue {
    Bool(bool),

    Byte(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    ISize(isize),

    UByte(u8),
    SmallUInt(u16),
    UInt(u32),
    BigUInt(u64),
    USize(usize),

    Float(f32),
    Double(f64),
    Char(char),
    String(String),
    // Decimal(Decimal),
    // DateTime(NaiveDateTime),
    // Date(NaiveDate),
    // Time(NaiveTime),
}

impl BindingValue {
    pub fn kind(&self) -> BindingKind {
        match self {
            BindingValue::Bool(_) => BindingKind::Bool,
            BindingValue::Byte(_)
            | BindingValue::SmallInt(_)
            | BindingValue::Int(_)
            | BindingValue::BigInt(_)
            | BindingValue::ISize(_)
            | BindingValue::UByte(_)
            | BindingValue::SmallUInt(_)
            | BindingValue::UInt(_)
            | BindingValue::BigUInt(_)
            | BindingValue::USize(_) => BindingKind::Fixed,

            BindingValue::Float(_) | BindingValue::Double(_) => BindingKind::Real,
            BindingValue::Char(_) | BindingValue::String(_) => BindingKind::Text,
            // BindingValue::Decimal(_) => BindingKind::Real,
            // BindingValue::DateTime(_) => BindingKind::DateTime,
            // BindingValue::Date(_) => BindingKind::Date,
            // BindingValue::Time(_) => BindingKind::Time,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BindingKind {
    Bool,
    Fixed,
    Real,
    Text,
    DateTime,
    Date,
    Time,
}

impl ToString for BindingValue {
    fn to_string(&self) -> String {
        match self {
            BindingValue::Bool(value) => value.to_string(),
            BindingValue::Byte(value) => value.to_string(),
            BindingValue::SmallInt(value) => value.to_string(),
            BindingValue::Int(value) => value.to_string(),
            BindingValue::BigInt(value) => value.to_string(),
            BindingValue::ISize(value) => value.to_string(),
            BindingValue::UByte(value) => value.to_string(),
            BindingValue::SmallUInt(value) => value.to_string(),
            BindingValue::UInt(value) => value.to_string(),
            BindingValue::BigUInt(value) => value.to_string(),
            BindingValue::USize(value) => value.to_string(),
            BindingValue::Float(value) => value.to_string(),
            BindingValue::Double(value) => value.to_string(),
            BindingValue::Char(value) => value.to_string(),
            BindingValue::String(value) => value.to_string(),
            //BindingValue::Decimal(value) => value.to_string(),
            //BindingValue::DateTime(value) => value.timestamp_nanos().to_string(),
            //BindingValue::Date(value) => value
            //    .and_time(NaiveTime::default())
            //    .timestamp_millis()
            //    .to_string(),
            //BindingValue::Time(value) => {
            //    (Decimal::new(NaiveDate::default().and_time(*value).timestamp_nanos(), 0)
            //        / rust_decimal_macros::dec!(60))
            //    .to_string()
            //}
        }
    }
}

impl From<&str> for BindingValue {
    fn from(value: &str) -> Self {
        BindingValue::String(value.to_owned())
    }
}

macro_rules! impl_from_binding_value {
    ($ty: ty, $ex: expr) => {
        impl From<$ty> for BindingValue {
            fn from(value: $ty) -> Self {
                $ex(value)
            }
        }
    };
}
impl_from_binding_value!(bool, BindingValue::Bool);
impl_from_binding_value!(i8, BindingValue::Byte);
impl_from_binding_value!(i16, BindingValue::SmallInt);
impl_from_binding_value!(i32, BindingValue::Int);
impl_from_binding_value!(i64, BindingValue::BigInt);
impl_from_binding_value!(isize, BindingValue::ISize);
impl_from_binding_value!(u8, BindingValue::UByte);
impl_from_binding_value!(u16, BindingValue::SmallUInt);
impl_from_binding_value!(u32, BindingValue::UInt);
impl_from_binding_value!(u64, BindingValue::BigUInt);
impl_from_binding_value!(usize, BindingValue::USize);
impl_from_binding_value!(f32, BindingValue::Float);
impl_from_binding_value!(f64, BindingValue::Double);
impl_from_binding_value!(char, BindingValue::Char);
impl_from_binding_value!(String, BindingValue::String);

//impl_from_binding_value!(Decimal, BindingValue::Decimal);
//impl_from_binding_value!(NaiveDateTime, BindingValue::DateTime);
//impl_from_binding_value!(NaiveDate, BindingValue::Date);
//impl_from_binding_value!(NaiveTime, BindingValue::Time);

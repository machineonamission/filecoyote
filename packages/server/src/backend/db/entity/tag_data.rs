use sea_orm::{ColIdx, ColumnType, DynIden, QueryResult, TryGetError, Value, sea_query};
use sea_orm::sea_query::{ArrayType, ValueTypeErr};
use sea_orm::ColumnType::Custom;

// fuckass way to hack in true "any" type support for sqlite <-> sea-orm boundary

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TagData {
    String(String),
    Int(i64),
    // Float(f64),  // floats dont fucking implement Eq
}

impl From<TagData> for Value {
    fn from(value: TagData) -> Self {
        match value {
            TagData::String(a) => Value::String(Some(a)),
            // TagData::Float(a) => Value::Double(Some(a)),
            TagData::Int(a) => Value::BigInt(Some(a)),
        }
    }
}

impl sea_query::ValueType for TagData {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(a)) => Ok(TagData::String(a)),
            Value::BigInt(Some(a)) => Ok(TagData::Int(a)),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "any".into()
    }

    fn array_type() -> ArrayType {
        // this is a postgres thing so it doesnt fuckin matter, just throw something here
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        Custom(DynIden::from("any"))
    }
}

impl sea_orm::TryGetable for TagData {
    fn try_get_by<I: ColIdx>(
        res: &QueryResult,
        idx: I,
    ) -> Result<Self, TryGetError> {
        match i64::try_get_by(res, idx) {
            Ok(a) => Ok(TagData::Int(a)),
            Err(b) => {
                let s = String::try_get_by(res, idx)?;
                Ok(TagData::String(s))
            }
        }
    }
}
use crate::schema::sql_types::DirectionType;
use crate::schema::sql_types::OrderTypeType;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::pg::PgValue;
use diesel::query_builder::QueryId;
use diesel::serialize;
use diesel::serialize::IsNull;
use diesel::serialize::Output;
use diesel::serialize::ToSql;
use diesel::AsExpression;
use diesel::FromSqlRow;
use std::any::TypeId;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = DirectionType)]
pub enum Direction {
    Long,
    Short,
}

impl QueryId for DirectionType {
    type QueryId = DirectionType;
    const HAS_STATIC_QUERY_ID: bool = false;

    fn query_id() -> Option<TypeId> {
        None
    }
}

impl ToSql<DirectionType, Pg> for Direction {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Direction::Long => out.write_all(b"long")?,
            Direction::Short => out.write_all(b"short")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<DirectionType, Pg> for Direction {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"long" => Ok(Direction::Long),
            b"short" => Ok(Direction::Short),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = OrderTypeType)]
pub enum OrderType {
    Market,
    Limit,
}

impl QueryId for OrderTypeType {
    type QueryId = OrderTypeType;
    const HAS_STATIC_QUERY_ID: bool = false;

    fn query_id() -> Option<TypeId> {
        None
    }
}

impl ToSql<OrderTypeType, Pg> for OrderType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            OrderType::Market => out.write_all(b"market")?,
            OrderType::Limit => out.write_all(b"limit")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<OrderTypeType, Pg> for OrderType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"market" => Ok(OrderType::Market),
            b"limit" => Ok(OrderType::Limit),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

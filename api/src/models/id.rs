//Adopted for MySQL from: https://github.com/forte-music/core/blob/fc9cd6217708b0dd6ae684df3a53276804479c59/src/models/id.rs

use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::expression::bound::Bound;
use diesel::expression::AsExpression;
use diesel::mysql::Mysql;
use diesel::serialize;
use diesel::serialize::Output;
use diesel::sql_types::Binary;
use diesel::sql_types::HasSqlType;
use diesel::types::ToSql;

use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;

use uuid;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Debug, AsExpression, FromSqlRow, Copy, Clone, Serialize, Deserialize)]
pub struct UUID(pub Uuid);

impl UUID {
    pub fn parse_str(input: &str) -> Result<UUID, uuid::ParseError> {
        Ok(UUID(Uuid::parse_str(input)?))
    }

    pub fn from_number(value: u64) -> Result<UUID, uuid::ParseError> {
        let bytes = number_to_arr(value);

        Ok(UUID(Uuid::from_bytes(&bytes)?))
    }
}

fn number_to_arr(value: u64) -> [u8; 16] {
    let mut bytes = [0; 16];
    for i in 0..(64 / 8) {
        bytes[16 - 1 - i] = ((value >> (8 * i)) & 0x000000ff) as u8;
    }

    bytes
}

#[test]
fn test_number_to_arr_zero() {
    assert_eq!(number_to_arr(0), [0; 16]);
}

#[test]
fn test_number_to_arr_mid() {
    assert_eq!(
        number_to_arr(270),
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 14]
    )
}

#[test]
fn test_number_to_arr_max() {
    let arr = number_to_arr(u64::max_value());
    assert_eq!(arr[8..16], [u8::max_value(); 8]);
    assert_eq!(arr[0..8], [0; 8]);
}

impl<DB: Backend + HasSqlType<Binary>> ToSql<Binary, DB> for UUID {
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let bytes = self.0.as_bytes();
        <[u8] as ToSql<Binary, DB>>::to_sql(bytes, out)
    }
}

impl FromSql<Binary, Mysql> for UUID {
    fn from_sql(bytes: Option<&<Mysql as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes_vec: Vec<u8> = <Vec<u8> as FromSql<Binary, Mysql>>::from_sql(bytes)?;
        Ok(UUID(Uuid::from_bytes(&bytes_vec)?))
    }
}

impl AsExpression<Binary> for UUID {
    type Expression = Bound<Binary, UUID>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<Binary> for &'a UUID {
    type Expression = Bound<Binary, &'a UUID>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl Hash for UUID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        let inner: Vec<Uuid> = data.iter().map(|s| s.0).collect();
        Uuid::hash_slice(inner.as_ref(), state);
    }
}

impl PartialEq for UUID {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

impl Eq for UUID {}

impl ToString for UUID {
    fn to_string(&self) -> String {
        self.0.simple().to_string()
    }
}

impl Into<UUID> for u64 {
    fn into(self) -> UUID {
        UUID::from_number(self).unwrap()
    }
}

impl From<rocket_contrib::uuid::Uuid> for UUID {
    fn from(other: rocket_contrib::uuid::Uuid) -> UUID {
        UUID(Uuid::from_bytes(other.as_bytes()).unwrap())
    }
}

/*
impl<'de> Deserialize<'de> for UUID {
    fn deserialize<D>(deserializer: D) -> Result<UUID, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        UUID::from_str(val).map_err(D::Error::custom)
    }
}

impl Serialize for UUID {
    fn serialize<S>(val: &UUID, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_string().serialize(serializer)
    }
}
*/

use crate::connectors::sql_shared::sql::SQLDialect;
use crate::core::field_type::FieldType;

// The database internal types.
#[derive(Debug, Clone)]
pub enum DatabaseType {

    // This value will be finally altered.
    Undefined,

    #[cfg(feature = "data-source-mongodb")]
    ObjectId,

    // In MySQL, it's alias for TINYINT(1).
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres", feature = "data-source-mongodb"))]
    Bool,

    // Bit(M), M is from 1 - 64. If M is omitted, M is 1 by default. To assign,
    // b'value' is used. For example, assigning a value of b'101' to a BIT(6) column is, in effect,
    // the same as assigning b'000101'.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Bit(u8),

    // PostgreSQL only
    #[cfg(feature = "data-source-postgres")]
    BitVarying,

    // TinyInt(signed), from -128 to 127. Unsigned version is from 0 - 255.
    // Available on MySQL only.
    #[cfg(feature = "data-source-mysql")]
    TinyInt(bool),

    // SmallInt(signed), from -32768 to 32767. Unsigned version is from 0 - 65535.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    SmallInt(bool),

    // MediumInt(M), from -8388608 to 8388607. Unsigned version is from 0 - 16777215.
    // Available for MySQL only.
    #[cfg(feature = "data-source-mysql")]
    MediumInt(bool),

    // Int(signed), from -2147483648 to 2147483647. Unsigned version is from 0 to 4294967295.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Int(bool),

    // BigInt(M), from -9223372036854775808 to 9223372036854775807. Unsigned version is from 0 to
    // 18446744073709551615.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres", feature = "data-source-mongodb"))]
    BigInt(bool),

    // MySQL: Decimal(M, D) PostgreSQL: Decimal(precision, scale)
    // A packed “exact” fixed-point number. M is the total number of digits (the precision) and D
    // is the number of digits after the decimal point (the scale). The decimal point and (for
    // negative numbers) the - sign are not counted in M. If D is 0, values have no decimal point
    // or fractional part. The maximum number of digits (M) for DECIMAL is 65. The maximum number
    // of supported decimals (D) is 30. If D is omitted, the default is 0. If M is omitted, the
    // default is 10.
    // Available for MySQL and PostgreSQL.
    // Numeric, Dec, Fixed are all the same.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres", feature = "data-source-mongodb"))]
    Decimal(Option<u8>, Option<u8>),

    // Float(p)
    // A floating-point number. p represents the precision in bits, but MySQL uses this value only
    // to determine whether to use FLOAT or DOUBLE for the resulting data type. If p is from 0 to
    // 24, the data type becomes FLOAT with no M or D values. If p is from 25 to 53, the data type
    // becomes DOUBLE with no M or D values.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Float(u8),

    // Double
    // A double precision. This name is remapped to DOUBLE PRECISION for PostgreSQL.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Double,

    // Real
    // A normal float in MySQL or real in PostgreSQL.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Real,

    // A date. In MySQL, the supported range is '1000-01-01' to '9999-12-31'. In MongoDB, this
    // represents datetime.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres", feature = "data-source-mongodb"))]
    Date,

    // datetime. fsp is from 0 - 6. The supported range is '1000-01-01 00:00:00.000000' to
    // '9999-12-31 23:59:59.999999'
    // This is MySQL only.
    #[cfg(feature = "data-source-mysql")]
    DateTime(u8),

    // Timestamp(p, with timezone)
    // A timestamp. In MySQL, the range is '1970-01-01 00:00:01.000000' UTC to
    // '2038-01-19 03:14:07.999999' UTC. In MySQL, the with timezone option is ignored.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres", feature = "data-source-mongodb"))]
    Timestamp(u8, bool),

    // Time(fsp, with timezone), fsp is from 0 - 6. Time zone is ignored for MySQL.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Time(u8, bool),

    // This is mysql only
    #[cfg(feature = "data-source-mysql")]
    Year,

    // String types

    // Char(len, charset, collate)
    // On PostgreSQL, charset and collate are ignored.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Char(u8, Option<String>, Option<String>),

    // VarChar(len, charset, collate)
    // On PostgreSQL, charset and collate are ignored.
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    VarChar(u16, Option<String>, Option<String>),

    // TinyText(charset, collate)
    #[cfg(feature = "data-source-mysql")]
    TinyText(Option<String>, Option<String>),

    // MediumText(charset, collate)
    #[cfg(feature = "data-source-mysql")]
    MediumText(Option<String>, Option<String>),

    // LongText(charset, collate)
    #[cfg(feature = "data-source-mysql")]
    LongText(Option<String>, Option<String>),

    // Text is different in MySQL and PostgreSQL
    #[cfg(all(feature = "data-source-mysql", feature = "data-source-postgres"))]
    Text(Option<u16>, Option<String>, Option<String>),

    #[cfg(feature = "data-source-mysql")]
    Binary(u8),

    #[cfg(feature = "data-source-mysql")]
    VarBinary(u16),

    #[cfg(feature = "data-source-mysql")]
    TinyBlob,

    #[cfg(feature = "data-source-mysql")]
    MediumBlob,

    #[cfg(feature = "data-source-mysql")]
    LongBlob,

    #[cfg(feature = "data-source-mysql")]
    Blob(u16),

    // ByteA type
    #[cfg(feature = "data-source-postgres")]
    ByteA,
}

impl Into<FieldType> for &DatabaseType {
    fn into(self) -> FieldType {
        match self {
            DatabaseType::Undefined => FieldType::Undefined,
            DatabaseType::ObjectId => FieldType::ObjectId,
            DatabaseType::Bool => FieldType::Bool,
            DatabaseType::Bit(_) => todo!(),
            DatabaseType::BitVarying => todo!(),
            DatabaseType::TinyInt(unsigned) => if *unsigned { FieldType::U8 } else { FieldType::I8 },
            DatabaseType::SmallInt(unsigned) => if *unsigned { FieldType::U16 } else { FieldType::I16 },
            DatabaseType::MediumInt(unsigned) => if *unsigned { FieldType::U32 } else { FieldType::I32 },
            DatabaseType::Int(unsigned) => if *unsigned { FieldType::U32 } else { FieldType::I32 },
            DatabaseType::BigInt(unsigned) => if *unsigned { FieldType::U64 } else { FieldType::I64 },
            DatabaseType::Decimal(_, _) => todo!(),
            DatabaseType::Float(precision) => if *precision >= 25 { FieldType::F64 } else { FieldType::F32 },
            DatabaseType::Double => FieldType::F64,
            DatabaseType::Real => FieldType::F32,
            DatabaseType::Date => FieldType::Date,
            DatabaseType::DateTime(_) => FieldType::DateTime,
            DatabaseType::Timestamp(_, _) => FieldType::DateTime,
            DatabaseType::Time(_, _) => todo!(),
            DatabaseType::Year => FieldType::String,
            DatabaseType::Char(_, _, _) => FieldType::String,
            DatabaseType::VarChar(_, _, _) => FieldType::String,
            DatabaseType::TinyText(_, _) => FieldType::String,
            DatabaseType::MediumText(_, _) => FieldType::String,
            DatabaseType::LongText(_, _) => FieldType::String,
            DatabaseType::Text(_, _, _) => FieldType::String,
            DatabaseType::Binary(_) => FieldType::String,
            DatabaseType::VarBinary(_) => FieldType::String,
            DatabaseType::TinyBlob => FieldType::String,
            DatabaseType::MediumBlob => FieldType::String,
            DatabaseType::LongBlob => FieldType::String,
            DatabaseType::Blob(_) => FieldType::String,
            DatabaseType::ByteA => FieldType::String,
        }
    }
}

impl DatabaseType {
    pub(crate) fn is_undefined(&self) -> bool {
        match self {
            DatabaseType::Undefined => true,
            _ => false
        }
    }

    pub(crate) fn to_string(&self, dialect: SQLDialect) -> String {
        match self {
            DatabaseType::Undefined => "Unimplemented".to_string(),
            DatabaseType::ObjectId => panic!(),
            DatabaseType::Bool => "Bool".to_string(),
            DatabaseType::Bit(l) => format!("BIT({l})"),
            DatabaseType::BitVarying => "BIT VARYING".to_string(),
            DatabaseType::TinyInt(u) => (if *u { "TINYINT UNSIGNED" } else { "TINYINT" }).to_string(),
            DatabaseType::SmallInt(u) => (if *u { "SMALLINT UNSIGNED" } else { "SMALLINT" }).to_string(),
            DatabaseType::MediumInt(u) => (if *u { "MEDIUMINT UNSIGNED" } else { "MEDIUMINT" }).to_string(),
            DatabaseType::Int(u) => (if *u { "INT UNSIGNED" } else { "INT" }).to_string(),
            DatabaseType::BigInt(u) => (if *u { "BIGINT UNSIGNED" } else { "BIGINT" }).to_string(),
            DatabaseType::Decimal(_, _) => todo!(),
            DatabaseType::Float(p) => format!("FLOAT({p})"),
            DatabaseType::Double => {
                if dialect == SQLDialect::PostgreSQL {
                    "DOUBLE PRECISION".to_string()
                } else {
                    "DOUBLE".to_string()
                }
            }
            DatabaseType::Real => {
                if dialect == SQLDialect::MySQL {
                    "FLOAT".to_string()
                } else {
                    "REAL".to_string()
                }
            }
            DatabaseType::Date => "DATE".to_string(),
            DatabaseType::DateTime(fsp) => format!("DATETIME({fsp})"),
            DatabaseType::Timestamp(fsp, tz) => {
                if dialect == SQLDialect::PostgreSQL {
                    let tzinfo = if *tz { " WITH TIME ZONE" } else { "" };
                    format!("TIMESTAMP({fsp}){tzinfo}")
                } else {
                    format!("TIMESTAMP({fsp})")
                }
            }
            DatabaseType::Time(_, _) => todo!(),
            DatabaseType::Year => "YEAR".to_string(),
            DatabaseType::Char(l, cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("CHAR({l}){charset}{collation}")
            }
            DatabaseType::VarChar(l, cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("VARCHAR({l}){charset}{collation}")
            }
            DatabaseType::TinyText(cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("TINYTEXT{charset}{collation}")
            }
            DatabaseType::MediumText(cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("MEDIUMTEXT{charset}{collation}")
            }
            DatabaseType::LongText(cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("LONGTEXT{charset}{collation}")
            }
            DatabaseType::Text(l, cs, co) => {
                if dialect == SQLDialect::PostgreSQL {
                    "TEXT".to_string()
                } else {
                    let charset = if let Some(v) = cs {
                        format!(" CHARACTER SET {v}")
                    } else { "".to_string() };
                    let collation = if let Some(v) = co {
                        format!(" COLLATION {v}")
                    } else { "".to_string() };
                    format!("TINYTEXT{charset}{collation}")
                }
            }
            DatabaseType::Binary(l) => format!("BINARY({l})"),
            DatabaseType::VarBinary(l) => format!("VARBINARY({l})"),
            DatabaseType::TinyBlob => "TINYBLOB".to_string(),
            DatabaseType::MediumBlob => "MEDIUMBLOB".to_string(),
            DatabaseType::LongBlob => "LONGBLOB".to_string(),
            DatabaseType::Blob(l) => format!("BLOB({l})"),
            DatabaseType::ByteA => "bytea".to_string(),
        }
    }
}

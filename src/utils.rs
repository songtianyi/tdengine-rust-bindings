use crate::bindings;

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_int, c_void};

// #[derive(Debug)]
pub enum Field {
    tinyInt(i8),
    smallInt(i16),
    normalInt(i32),
    bigInt(i64),
    float(f32),
    double(f64),
    string(String),
    boolType(bool),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Field::tinyInt(v) => write!(f, "{}", v),
            Field::smallInt(v) => write!(f, "{}", v),
            Field::normalInt(v) => write!(f, "{}", v),
            Field::bigInt(v) => write!(f, "{}", v),
            Field::float(v) => write!(f, "{}", v),
            Field::double(v) => write!(f, "{}", v),
            Field::tinyInt(v) => write!(f, "{}", v),
            Field::boolType(v) => write!(f, "{}", v),
            Field::string(v) => write!(f, "{}", v),
        }
    }
}

macro_rules! impl_as_fields {
    ($fn:ident, $pattern:pat => $v:expr, $type:ty) => {
        pub fn $fn(&self) -> $type {
            match *self {
                $pattern => $v,
                _ => {
                    println!("unexpected $type value {}", self);
                    Default::default()
                }
            }
        }
    };
}

impl Field {
    impl_as_fields!(as_i8, Field::tinyInt(v) => v, i8);
    impl_as_fields!(as_i16, Field::smallInt(v) => v, i16);
    impl_as_fields!(as_i32, Field::normalInt(v) => v, i32);
    impl_as_fields!(as_i64, Field::bigInt(v) => v, i64);
    impl_as_fields!(as_f32, Field::float(v) => v, f32);
    impl_as_fields!(as_f64, Field::double(v) => v, f64);
    impl_as_fields!(as_bool, Field::boolType(v) => v, bool);

    pub fn as_string(&self) -> String {
        match &*self {
            Field::string(v) => v.to_string(),
            _ => {
                println!("unexpected string value {}", self);
                "".to_string()
            }
        }
    }
}

pub type Row = Vec<Field>;

pub fn format_row(row: &Row) -> String {
    let mut s = String::new();
    for field in row {
        s.push_str(format!("{} ", field).as_str());
    }
    s
}

pub fn str_into_raw(s: &str) -> *mut c_char {
    if s.is_empty() {
        0 as *mut c_char
    } else {
        CString::new(s).unwrap().into_raw()
    }
}

pub fn raw_into_str<'a>(raw: *mut c_char) -> &'static str {
    unsafe { CStr::from_ptr(raw).to_str().unwrap() }
}

pub fn raw_into_field(raw: *mut bindings::TAOS_FIELD, fcount: c_int) -> Vec<bindings::taosField> {
    let mut fields: Vec<bindings::taosField> = Vec::new();

    for i in 0..fcount as isize {
        fields.push(bindings::taosField {
            name: unsafe { *raw.offset(i as isize) }.name,
            bytes: unsafe { *raw.offset(i as isize) }.bytes,
            type_: unsafe { *raw.offset(i as isize) }.type_,
        });
    }
    fields
}

pub fn raw_into_row(
    fields: *mut bindings::TAOS_FIELD,
    fcount: c_int,
    raw_row: &[*mut c_void],
) -> Row {
    let mut row: Row = Vec::new();
    let fields = raw_into_field(fields, fcount);

    for (i, field) in fields.iter().enumerate() {
        unsafe {
            match field.type_ as u32 {
                bindings::TSDB_DATA_TYPE_TINYINT => {
                    row.push(Field::tinyInt(*(raw_row[i] as *mut i8)));
                }
                bindings::TSDB_DATA_TYPE_SMALLINT => {
                    row.push(Field::smallInt(*(raw_row[i] as *mut i16)));
                }
                bindings::TSDB_DATA_TYPE_INT => {
                    row.push(Field::normalInt(*(raw_row[i] as *mut i32)));
                }
                bindings::TSDB_DATA_TYPE_BIGINT | bindings::TSDB_DATA_TYPE_TIMESTAMP => {
                    row.push(Field::bigInt(*(raw_row[i] as *mut i64)));
                }
                bindings::TSDB_DATA_TYPE_FLOAT => {
                    row.push(Field::float(*(raw_row[i] as *mut f32)));
                }
                bindings::TSDB_DATA_TYPE_DOUBLE => {
                    row.push(Field::double(*(raw_row[i] as *mut f64)));
                }
                bindings::TSDB_DATA_TYPE_NCHAR => {
                    let a = CStr::from_ptr(raw_row[i] as *const c_char)
                        .to_string_lossy()
                        .into_owned();
                    row.push(Field::string(a));
                }
                bindings::TSDB_DATA_TYPE_BOOL => {
                    row.push(Field::boolType(*(raw_row[i] as *mut i8) != 0));
                }
                _ => println!(""),
            }
        }
    }
    row
}

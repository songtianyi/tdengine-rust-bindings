use crate::bindings::*;
use crate::utils;

use std::os::raw::{c_int, c_void};
use std::ptr;

pub struct Subscriber {
    tsub: *mut c_void,
    fields: *mut taosField,
    fcount: c_int,
}

impl Subscriber {
    pub fn subscribe(
        taos: *mut c_void,
        restart: i32,
        topic: &str,
        sql: &str,
        interval: i32,
    ) -> Result<Subscriber, &'static str> {
        unsafe {
            let mut tsub = taos_subscribe(
                taos,
                restart as c_int,
                utils::str_into_raw(topic),
                utils::str_into_raw(sql),
                None,
                ptr::null_mut(),
                interval as c_int,
            );
            if tsub.is_null() {
                return Err(utils::raw_into_str(taos_errstr(tsub)));
            }

            Ok(Subscriber {
                tsub: tsub,
                fields: ptr::null_mut(), // init
                fcount: 0,               // init
            })
        }
    }

    pub fn consume(self: &mut Subscriber) -> Result<Vec<utils::Row>, &'static str> {
        unsafe {
            let taos_res = taos_consume(self.tsub);
            if taos_res.is_null() {
                return Err(utils::raw_into_str(taos_errstr(self.tsub)));
            }
            if self.fields.is_null() {
                self.fields = taos_fetch_fields(taos_res);
                // fetch err msg
                if self.fields.is_null() {
                    return Err(utils::raw_into_str(taos_errstr(taos_res)));
                }
            }

            if self.fcount == 0 {
                self.fcount = taos_field_count(taos_res);
                if self.fcount == 0 {
                    return Err(utils::raw_into_str(taos_errstr(taos_res)));
                }
                println!("{} fields", self.fcount);
            }

            let mut rows: Vec<utils::Row> = Vec::<utils::Row>::new();
            loop {
                let taos_row = taos_fetch_row(taos_res);
                if taos_row.is_null() {
                    // chek retcode
                    if taos_errno(taos_res) != 0 {
                        return Err(utils::raw_into_str(taos_errstr(taos_res)));
                    }
                    break;
                }

                // example code for taos_print_row
                //let mut buf: Vec<c_char> = vec![0; 4096];
                //taos_print_row(buf.as_mut_ptr(), taos_row, self.fields, self.fcount);
                //println!(
                //    "internal print {}",
                //    CStr::from_ptr(buf.as_ptr()).to_str().unwrap()
                //);

                let raw_row = std::slice::from_raw_parts(taos_row, self.fcount as usize);
                let row = utils::raw_into_row(self.fields, self.fcount, raw_row);
                rows.push(row);
            }
            println!("{} rows fetched", rows.len());
            Ok(rows)
        }
    }

    pub fn unsubscribe(self: &Subscriber, keep_progress: i32) {
        unsafe { taos_unsubscribe(self.tsub, keep_progress as c_int) }
    }

    pub fn print_row(self: &Subscriber, row: &utils::Row) {
        println!("format row: {}", utils::format_row(row));
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        unsafe {
            taos_unsubscribe(self.tsub, 1);
        }
    }
}

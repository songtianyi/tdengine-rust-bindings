use crate::bindings::*;
use crate::utils;

use std::os::raw::c_void;

pub struct Tdengine {
    conn: *mut c_void, // tdengine conn
}

impl Tdengine {
    pub fn new(
        ip: &str, // host ip
        username: &str,
        passwd: &str,
        db: &str,
        port: i32, // tdengine port
    ) -> Result<Tdengine, &'static str> {
        unsafe {
            taos_init();
            let mut conn = taos_connect(
                utils::str_into_raw(ip),
                utils::str_into_raw(username),
                utils::str_into_raw(passwd),
                utils::str_into_raw(db),
                port as u16,
            );
            if conn.is_null() {
                Err(utils::raw_into_str(taos_errstr(conn)))
            } else {
                println!("connected to {}:{} user:{}, db:{}", ip, port, username, db);
                Ok(Tdengine { conn })
            }
        }
    }

    pub fn query(self: &Tdengine, s: &str) -> Result<Vec<utils::Row>, &'static str> {
        unsafe {
            let taos_res = taos_query(self.conn, utils::str_into_raw(s));
            // check retcode
            if taos_errno(taos_res) != 0 {
                let err = utils::raw_into_str(taos_errstr(taos_res));
                println!("query '{}' error: {}", s, err);
                return Err(err);
            }
            println!("query '{}' ok", s);
            let mut rows: Vec<utils::Row> = Vec::<utils::Row>::new();
            let fields = taos_fetch_fields(taos_res);
            let fcount = taos_field_count(taos_res);
            loop {
                let taos_row = taos_fetch_row(taos_res);
                if taos_row.is_null() {
                    // check retcode
                    if taos_errno(taos_res) != 0 {
                        return Err(utils::raw_into_str(taos_errstr(taos_res)));
                    }
                    break;
                }

                let raw_row = std::slice::from_raw_parts(taos_row, fcount as usize);
                let row = utils::raw_into_row(fields, fcount, raw_row);
                rows.push(row);
            }
            taos_free_result(taos_res);
            println!("{} rows fetched", rows.len());
            Ok(rows)
        }
    }

    pub fn taos(self: &Tdengine) -> *mut c_void {
        self.conn
    }
}

impl Drop for Tdengine {
    fn drop(&mut self) {
        unsafe {
            taos_close(self.conn);
            // we will not do clean up here
        }
    }
}

pub fn clean_up() {
    unsafe {
        taos_cleanup();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

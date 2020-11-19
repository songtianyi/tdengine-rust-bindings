use libtdengine::tdengine::{clean_up, Tdengine};
use libtdengine::utils;
use std::process;

fn main() {
    let tde = Tdengine::new("127.0.0.1:6030", "root", "taosdata", "log", 0).unwrap_or_else(|err| {
        eprintln!("Can't create Tdengine: {}", err);
        process::exit(1)
    });

    assert_eq!(tde.query("drop database demo").is_ok(), true);
    assert_eq!(tde.query("create database demo").is_ok(), true);
    assert_eq!(tde.query("use demo").is_ok(), true);
    assert_eq!(
        tde.query("create table m1 (ts timestamp, speed int)")
            .is_ok(),
        true
    );

    for i in 0..10 {
        assert_eq!(
            tde.query(format!("insert into m1 values (now+{}s, {})", i, i).as_str())
                .is_ok(),
            true
        );
    }
    let rows = tde.query("select * from m1").unwrap_or_else(|err| {
        eprintln!("select error: {}", err);
        process::exit(1);
    });
    for row in rows {
        println!("{}", utils::format_row(&row));
    }
    // drop manually before clean up
    drop(tde);
    // cleanup before program exit
    clean_up();
}

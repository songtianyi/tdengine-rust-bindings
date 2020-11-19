use libtdengine::subscriber::Subscriber;
use libtdengine::tdengine::{clean_up, Tdengine};
use std::process;

fn main() {
    let td = Tdengine::new("127.0.0.1", "root", "taosdata", "demo", 0).unwrap_or_else(|err| {
        eprintln!("Can't create Tdengine: {}", err);
        process::exit(1)
    });

    let mut subscriber =
        Subscriber::subscribe(td.taos(), 0, "topic_test", "select * from m1;", 1000)
            .unwrap_or_else(|err| {
                eprintln!("Can't create Subscriber: {}", err);
                process::exit(1)
            });

    loop {
        let rows = match subscriber.consume() {
            Ok(rows) => rows,
            Err(err) => {
                eprintln!("consume exit: {}", err);
                break;
            }
        };

        // example code
        for row in rows {
            println!("({}, {})", row[0].as_i64(), row[1].as_i32());
        }
        println!("====================");
    }
    drop(td);
    clean_up();
}

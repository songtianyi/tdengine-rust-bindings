# TDengine rust bindings

It's a rust bindings project for [TDengine](https://github.com/taosdata/TDengine)

AND it's based on OUTDATED [tdengine-connector](https://github.com/tidyjiang8/tdengine-connector), thanks for [@tidyjiang8](https://github.com/tidyjiang8)'s prior work.

## Dependencies
- [Tdengine](https://www.taosdata.com/en/getting-started/)
- [Rust](https://www.rust-lang.org/learn/get-started)
- clang 3.9+
- cargo

## How to use 
The rust binding file of taos.h already generated, but you can try it with
```
cargo +nightly build
```
Then you find generated `bindings.rs`
```
find . -name "*.rs"
```
Copy to src folder
```
cp /path/to/bindings.rs src/
```
Run example `demo` to prepare demo data
> NOTE. `demo` will drop database 'demo' and create a new one.
```
cargo +nightly run --example demo
```
Then run example `subscribe` to test demo subscriber and check outputs
```
cargo +nightly run --example subscribe
```

## TDegnine API coverage
- taos_init
- taos_connect
- taos_close
- taos_query
   - taos_free_result
- taos_subscribe (callback function not support yet)
- taos_unsubscribe
- taos_consume
  - taos_fetch_fields
  - taos_field_count
  - taos_fetch_row
  - taos_errstr
  - taos_errno


## Platform coverage
This binding has only been tested in following platforms

- CentOS-7 (kernel-3.10.0-957.27.2.el7.x86_64)

let me know if it's running well in your platform or just edit this list.

## Contribution
We should keep track of TDengine latest c interfaces and support as more interfaces as we need.


## License
Keep same with [Tdengine](https://www.taosdata.com/en/getting-started/).



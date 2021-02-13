
```
error: target is not supported, for more information see: https://docs.rs/getrandom/#unsupported-targets
   --> /home/mina/.cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.2/src/lib.rs:213:9
    |
213 | /         compile_error!("target is not supported, for more information see: \
214 | |                         https://docs.rs/getrandom/#unsupported-targets");
    | |_________________________________________________________________________^

error[E0433]: failed to resolve: use of undeclared crate or module `imp`
   --> /home/mina/.cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.2/src/lib.rs:235:5
    |
235 |     imp::getrandom_inner(dest)
    |     ^^^ use of undeclared crate or module `imp`
```


[dependencies]
getrandom = { version = "0.2", feature = ["js"] }
js-sys = "0.3.47"
imp = "0.1.0"
lerp = "0.4.0"
noise = "0.7.0"
# rand = { version = "0.8.3", feature = ["getrandom"] }
rand_core = { version = "0.6.0", feature = ["getrandom"] }
wasm-bindgen = "0.2.70"


use rand_core::OsRng;
        let mut rng = OsRng::new();
        // let mut rng = match OsRng::new() {
        //     Ok(g) => g,
        //     Err(e) => panic!("Failed to obtain OS RNG: {}", e),
        // };
        let adj: u32 = rng.next_u32();

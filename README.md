This looks like a build bug.

```
   Compiling mylib v0.1.0 (file:///home/remram/Documents/programming/rustbug/testlink-rs)
mylib/src/stuff.rs:2:1: 2:20 warning: constant item is never used: `ADD`, #[warn(dead_code)] on by default
mylib/src/stuff.rs:2 const ADD: u32 = 3;  // REPORTED AS DEAD CODE
                     ^~~~~~~~~~~~~~~~~~~
mylib/src/stuff.rs:18:5: 23:6 warning: method is never used: `foo`, #[warn(dead_code)] on by default
mylib/src/stuff.rs:18     pub fn foo(&self) {  // REPORTED AS DEAD CODE
mylib/src/stuff.rs:19         println!("{}", match *self {
mylib/src/stuff.rs:20             MyType::Yes => 4,
mylib/src/stuff.rs:21             MyType::No(nb) => nb + ADD
mylib/src/stuff.rs:22         });
mylib/src/stuff.rs:23     }
   Compiling testlink v0.1.0 (file:///home/remram/Documents/programming/rustbug/testlink-rs)
error: linking with `cc` failed: exit code: 1
note: "cc" "-Wl,--as-needed" "-m64" "-L" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/remram/Documents/programming/rustbug/testlink-rs/target/debug/testlink.o" "-o" "/home/remram/Documents/programming/rustbug/testlink-rs/target/debug/testlink" "-Wl,--whole-archive" "-l" "morestack" "-Wl,--no-whole-archive" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "/home/remram/Documents/programming/rustbug/testlink-rs/target/debug/deps/libmylib-8d142e20c60dadc2.rlib" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-74fa456f.rlib" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-74fa456f.rlib" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-74fa456f.rlib" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-74fa456f.rlib" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-74fa456f.rlib" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-74fa456f.rlib" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-74fa456f.rlib" "-L" "/home/remram/Documents/programming/rustbug/testlink-rs/target/debug" "-L" "/home/remram/Documents/programming/rustbug/testlink-rs/target/debug/deps" "-L" "/home/remram/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/remram/Documents/programming/rustbug/testlink-rs/.rust/lib/x86_64-unknown-linux-gnu" "-L" "/home/remram/Documents/programming/rustbug/testlink-rs/lib/x86_64-unknown-linux-gnu" "-Wl,-Bstatic" "-Wl,-Bdynamic" "-l" "dl" "-l" "pthread" "-l" "rt" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-Wl,-rpath,$ORIGIN/../../../../../../.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "compiler-rt"
note: /home/remram/Documents/programming/rustbug/testlink-rs/target/debug/testlink.o: In function `testlink::main':
/home/remram/Documents/programming/rustbug/testlink-rs/src/main.rs:7: undefined reference to `stuff::MyType::foo::h24e15252107abef3Laa'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
Could not compile `testlink`.

To learn more, run the command again with --verbose.
```

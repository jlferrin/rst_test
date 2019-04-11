
----
## Debugging session example: testgdb

### Codigo

``` rust
// pretty.rs
const COLORS: [&'static str;7] =
  ["red", "yellow", "pink", "green", "purple", "orange", "blue"];
struct Label {
    index: usize,
    color: &'static str
}
fn main() {
    // create a Vec of Label
    let labels: Vec<Label> = (0..10).map(|i| {
        Label { index: i, color: COLORS[i % COLORS.len()] }
      }).collect();
    // print them
    for label in labels {
        println!("{}: {}", label.index, label.color);
    }
}
```

### Configuración: Usando profiles:

```
$ cat Cargo.toml
[package]
name = "testdbg"
version = "0.1.0"
authors = ["juanillo"]
edition = "2018"

[profile.dev]
opt-level = 0  # Controls the --opt-level the compiler builds with
debug = true   # Controls whether the compiler passes `-g`
# The release profile, used for `cargo build --release`

[profile.release]
opt-level = 3
debug = false

[dependencies]

$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
```


### Usando gdb

``` bash
$ rust-gdb target/
debug/            .rustc_info.json
[juanillo@muralla mystr]$ rust-gdb target/debug/mystr
GNU gdb (GDB) 8.2
Copyright (C) 2018 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/debug/mystr...done.
(gdb)
```

### Comandos

Listar código

``` gdb
(gdb) l
1       // pretty.rs
2       const COLORS: [&'static str;7] =
3         ["red", "yellow", "pink", "green", "purple", "orange", "blue"];
4       struct Label {
5           index: usize,
6           color: &'static str
7       }
8       fn main() {
9           // create a Vec of Label
10          let labels: Vec<Label> = (0..10).map(|i| {
(gdb) l 11
6           color: &'static str
7       }
8       fn main() {
9           // create a Vec of Label
10          let labels: Vec<Label> = (0..10).map(|i| {
11              Label { index: i, color: COLORS[i % COLORS.len()] }
12            }).collect();
13          // print them
14          for label in labels {
15              println!("{}: {}", label.index, label.color);
```

Listar funciones

``` gdb
(gdb) info functions testdbg
All functions matching regular expression "testdbg":

File src/main.rs:
8:      static fn testdbg::main();
10:     static fn testdbg::main::{{closure}}(closure *, usize) -> testdbg::Label;
```

Usando breakpoints con nombre función

```gdb
(gdb) b testdbg::main
Breakpoint 1 at 0x9e37: file src/main.rs, line 10.
```

O con regexp:

```gdb
(gdb) rbreak .*main.*
Note: breakpoint 1 also set at pc 0x9e37.
Breakpoint 2 at 0x9e37: file src/main.rs, line 10.
static fn testdbg::main();
Breakpoint 3 at 0x748e: file src/main.rs, line 11.
static fn testdbg::main::{{closure}}(closure *, usize) -> testdbg::Label;
Breakpoint 4 at 0x5c50
<function, no debug info> __libc_start_main@plt;
Breakpoint 5 at 0xa130
<function, no debug info> main;
```


Sesión de debug

```gdb
(gdb) b testdbg::main
Breakpoint 1 at 0x9e37: file src/main.rs, line 10.

(gdb) r
Starting program: /home/juanillo/rust/tests/testdbg/target/debug/testdbg
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, testdbg::main () at src/main.rs:10
10          let labels: Vec<Label> = (0..10).map(|i| {

(gdb) bt
#0  testdbg::main () at src/main.rs:10

(gdb) n
14          for label in labels {

(gdb) n
15              println!("{}: {}", label.index, label.color);

(gdb) p label
$1 = testdbg::Label {index: 0, color: <error reading variable>}

(gdb) n
0: red
14          for label in labels {

(gdb) continue
Continuing.
1: yellow
2: pink
3: green
4: purple
5: orange
6: blue
7: red
8: yellow
9: pink
[Inferior 1 (process 1851) exited normally]

```


``` gdb
$ rust-gdb -q target/debug/mydebug
Reading symbols from target/debug/mydebug...done.

(gdb) b mydebug::main
Breakpoint 1 at 0x7017: file src/main.rs, line 10.

(gdb) r
Starting program: /home/juanillo/rust/rst_tests/mydebug/target/debug/mydebug
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, mydebug::main () at src/main.rs:10
10          let labels: Vec<Label> = (0..10).map(|i| {

(gdb) n
14          for label in labels {

(gdb) n
15              println!("{}: {}", label.index, label.color);

(gdb) p label
$1 = mydebug::Label {index: 0, color: <error reading variable>}

(gdb) p COLORS
No symbol 'COLORS' in current context

(gdb) info breakpoints
Num     Type           Disp Enb Address            What
1       breakpoint     keep y   0x000055555555b017 in mydebug::main at src/main.rs:10
        breakpoint already hit 1 time

(gdb) info locals
label = mydebug::Label {index: 0, color: <error reading variable>}
__next = mydebug::Label {index: 0, color: <error reading variable>}
iter = alloc::vec::IntoIter<mydebug::Label> {buf: core::ptr::NonNull<mydebug::Label> {pointer: core::nonzero::NonZero<*const mydebug::Label> (0x55555578d140)}, phantom: core::marker::PhantomData<mydebug::Label>, cap: 10, ptr: 0x55555578d158, end: 0x55555578d230}
labels = Vec<mydebug::Label>(len: 10, cap: 10) = {mydebug::Label {index: 0, color: <error reading variable>}, mydebug::Label {index: 1, color: <error reading variable>},
  mydebug::Label {index: 2, color: <error reading variable>}, mydebug::Label {index: 3, color: <error reading variable>}, mydebug::Label {index: 4, color: <error reading variable>},
  mydebug::Label {index: 5, color: <error reading variable>}, mydebug::Label {index: 6, color: <error reading variable>}, mydebug::Label {index: 7, color: <error reading variable>},
  mydebug::Label {index: 8, color: <error reading variable>}, mydebug::Label {index: 9, color: <error reading variable>}}

...

(gdb) n
17          let test = "hola mundo";

(gdb) n
18          println!("{}", test);

(gdb) p test
$4 = "hola mundo"

(gdb) continue
Continuing.
hola mundo
[Inferior 1 (process 2012) exited normally]

```

Se cambia:

```rust
//const COLORS: [&'static str;7] =
const COLORS: [&str;7] =
```

``` gdb
$ rust-gdb -q target/debug/mydebug
Reading symbols from target/debug/mydebug...done.
(gdb) b mydebug::main
Breakpoint 1 at 0x7017: file src/main.rs, line 11.
(gdb) r
Starting program: /home/juanillo/rust/rst_tests/mydebug/target/debug/mydebug
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, mydebug::main () at src/main.rs:11
11          let labels: Vec<Label> = (0..10).map(|i| {

(gdb) n
15          for label in labels {

(gdb) n
16              println!("{}: {}", label.index, label.color);

(gdb) p label
$1 = mydebug::Label {index: 0, color: <error reading variable>}

(gdb) p labels
$2 = Vec<mydebug::Label>(len: 10, cap: 10) = {mydebug::Label {index: 0, color: <error reading variable>}, mydebug::Label {index: 1, color: <error reading variable>},
  mydebug::Label {index: 2, color: <error reading variable>}, mydebug::Label {index: 3, color: <error reading variable>}, mydebug::Label {index: 4, color: <error reading variable>},
  mydebug::Label {index: 5, color: <error reading variable>}, mydebug::Label {index: 6, color: <error reading variable>}, mydebug::Label {index: 7, color: <error reading variable>},
  mydebug::Label {index: 8, color: <error reading variable>}, mydebug::Label {index: 9, color: <error reading variable>}}
(gdb)

```

El problema no son las constantes, es el struct con <'a> (seguir investigando):

```rust
// pretty.rs
//const COLORS: [&'static str;7] =
const COLORS: [&str;7] =
  ["red", "yellow", "pink", "green", "purple", "orange", "blue"];

const TEST: i64 = 300;

const TESTV: [i64;4] = [ 3,5,6,8];

const COLOR: &str = "cyan";

struct Label {
    index: usize,
    color: &'static str
}

struct Label2<'a> {
    index: usize,
    color: &'a str
}

struct Label3 {
    index: usize,
    color: String
}

fn main() {
    // create a Vec of Label
    let labels: Vec<Label> = (0..10).map(|i| {
        Label { index: i, color: COLORS[i % COLORS.len()] }
      }).collect();
    let test = TEST;
    let color = COLOR;
    let tests: Vec<i64> = TESTV.to_vec();
    let colors: Vec<&str> = COLORS.to_vec();
    let color2 = Label2 { index: 1, color: "magenta" };
    let color3 = Label3 { index: 1, color: "magenta".to_string() };

    // print them
    for label in labels {
        println!("{}: {}", label.index, label.color);
    }

    let anothertest = "hola mundo";
    println!("{} {}", test, anothertest);
}
```

```gdb
$ rust-gdb -q target/debug/mydebug
Reading symbols from target/debug/mydebug...done.
(gdb) b mydebug::main
Breakpoint 1 at 0x7d67: file src/main.rs, line 29.
(gdb) r
Starting program: /home/juanillo/rust/rst_tests/mydebug/target/debug/mydebug
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, mydebug::main () at src/main.rs:29
29          let labels: Vec<Label> = (0..10).map(|i| {
(gdb) n
32          let test = TEST;
(gdb) n
33          let color = COLOR;
(gdb) n
34          let tests: Vec<i64> = TESTV.to_vec();
(gdb) n
35          let colors: Vec<&str> = COLORS.to_vec();
(gdb) n
36          let color2 = Label2 { index: 1, color: "magenta" };
(gdb) n
37          let color3 = Label3 { index: 1, color: "magenta".to_string() };
(gdb) n
40          for label in labels {
(gdb) p color3
$1 = mydebug::Label3 {index: 1, color: "magenta"}
(gdb) p color2
$2 = mydebug::Label2 {index: 1, color: <error reading variable>}
(gdb) p colors
$3 = Vec<&str>(len: 7, cap: 7) = {"red", "yellow", "pink", "green", "purple", "orange", "blue"}
(gdb) p tests
$4 = Vec<i64>(len: 4, cap: 4) = {3, 5, 6, 8}
(gdb) p color
$5 = "cyan"
(gdb) p test
$6 = 300
(gdb) p labels
$7 = Vec<mydebug::Label>(len: 10, cap: 10) = {mydebug::Label {index: 0, color: <error reading variable>}, mydebug::Label {index: 1, color: <error reading variable>},
  mydebug::Label {index: 2, color: <error reading variable>}, mydebug::Label {index: 3, color: <error reading variable>}, mydebug::Label {index: 4, color: <error reading variable>},
  mydebug::Label {index: 5, color: <error reading variable>}, mydebug::Label {index: 6, color: <error reading variable>}, mydebug::Label {index: 7, color: <error reading variable>},
  mydebug::Label {index: 8, color: <error reading variable>}, mydebug::Label {index: 9, color: <error reading variable>}}
```


```
```

```
```
```
```
```
```
```
```
```
```
```
```
```
```
```
```
```
```
```
```
```

### Cargo

```bash
$ cargo new 

$ cd my_2ndapp

my_2ndapp $ cat src/main.rs
fn main() {
    println!("Hello, world!");
}

my_2ndapp $ cargo run
   Compiling my_2ndapp v0.1.0 (/home/juanillo/rust/my_2ndapp)
    Finished dev [unoptimized + debuginfo] target(s) in 3.38s
     Running `target/debug/my_2ndapp`
Hello, world!
```

### Variables, constantes, etc

Error, las variables son inmutables:
```rust
let x = 5;
x = 6;
```

Variables mutables:
```rust
let mut x = 5;
x = 6;
```

Constantes:
```rust
const DOCENA: u32 = 12;
```

Shadowing:
```rust
let x = 5;
let x = 6;
```

### Tipos

**Enteros**

<table>
    <tr>
<td>Length</td><td>Signed</td><td>Unsigned</td>
    </tr><tr>
<td>8-bit</td><td>i8</td><td>u8</td>
    </tr><tr>
<td>16-bit</td><td>i16</td><td>u16</td>
    </tr><tr>
<td>32-bit</td><td>i32</td><td>u32</td>
    </tr><tr>
<td>64-bit</td><td>i64</td><td>u64</td>
    </tr><tr>
<td>128-bit</td><td>i128</td><td>u128</td>
    </tr><tr>
<td>arch</td><td>isize</td><td>usize</td>
    </tr>
</table>

<table>
    <tr>
<td>Number literals</td><td>Example</td>
    </tr><tr>
<td>Decimal</td><td>98_222</td>
    </tr><tr>
<td>Hex</td><td>0xff</td>
    </tr><tr>
<td>Octal</td><td>0o77</td>
    </tr><tr>
<td>Binary</td><td>0b1111_0000</td>
    </tr><tr>
<td>Byte (u8 only)</td><td>b'A'</td>
    </tr>
</table>

Overflow de enteros:

u8 (0-255) = 256 --> Debug mode: error. Release: 0 (257 -> 1, etc)

**Coma Flotante**

```rust
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
```

Operaciones: + - * / %

**Boolean**

```rust
    let t = true;
    let f: bool = false; // with explicit type annotation
```

**Caracter**

char --> Unicode 

```rust
    let c = 'z'; // char
    let z = "Z"; // string
```

**Tuple**

```rust
    let tupla: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    
    let five_hundred = tupla.0;
    let six_point_four = tupla.1;
    let one = tupla.2;
```

**Array**

```rust
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
	      
    let first = a[0];
    let second = a[1];
```

### Funciones

Rust code uses snake case as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words.

In function signatures, you must declare the type of each parameter.

```rust
fn main() {
    println!("Hello, world!");
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

**Statements and Expressions**

Function bodies are made up of a series of statements optionally ending in an expression.

Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

```rust
fn main() {
    let x = 5; // Statement

    let y = {
        let x = 3;
        x + 1  // Expression: no semicolon
    };

    println!("The value of y is: {}", y);
}
```

Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

**Function with return Values**

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

### Control de flujo

**if**

You must be explicit and always provide if with a Boolean as its condition.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

if con let

```rust
    let number = if condition {
        5
    } else {
        6
    };
```

**loop**

One of the uses of a loop is to retry an operation you know can fail, such as checking if a thread completed its job. However, you might need to pass the result of that operation to the rest of your code. If you add it to the break expression you use to stop the loop, it will be returned by the broken loop:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

**while**

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

**for**

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```











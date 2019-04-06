use std::env;

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let hola = String::from("hola mondo");
//    let hola2 = String::from("hola mondo");

    println!("hola: {:p}",hola.as_ptr());

    let config1 = arg1(&args);
    println!("args config1: {:p},{:p}",args[1].as_ptr(),config1.as_ptr());

    let config2 = arg2(&args);
    println!("args config2: {:p},{:p}",args[1].as_ptr(),config2.as_ptr());

    let config4 = arg4(&hola[..]);// hola sigue
    println!("config4 hola: {:p},{:p}",config4.as_ptr(),hola.as_ptr());

    let config5 = arg5(&hola[..]);// hola sigue
    println!("config5 hola: {:p},{:p}",config5.as_ptr(),hola.as_ptr());

    let config3 = arg3(hola); // hola deja de existir
    println!("config3: {:p}",config3.as_ptr());

//    println!("config4: {:p}",config4.as_ptr()); // config4 marchó con hola
//    println!("config5: {:p}",config5.as_ptr()); // config5 marchó con hola

    // Access constant in the main thread
    let n=16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // THRESHOLD = 5; Error! Cannot modify a `const`.

}

fn arg1(args: &[String]) -> &String {
    let filename = &args[1];
    println!("arg1: {},{}", args[1], &args[1]);
    filename
}

fn arg2(args: &[String]) -> &str {
    let filename = &args[1];
    filename
}

fn arg3(args: String ) -> String {
    let filename = args; // args deja de existir!
    println!("arg1: {:p}", filename.as_ptr());
    filename
}

fn arg4(args: &str ) -> &str {
    let filename = args; 
    println!("arg1: {:p}", filename.as_ptr());
    &args[..]
}

fn arg5(args: &str ) -> &str {
    let filename = args; 
    println!("arg1: {:p}", filename.as_ptr());
    filename
}


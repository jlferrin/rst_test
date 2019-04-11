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

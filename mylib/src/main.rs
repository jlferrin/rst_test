
// -- Module with private an public 

mod sound {
   pub mod instrument {
       pub fn clarinet() -> String {
           let clarinet = "clarinet";
           super::print_val(clarinet);
           clarinet.to_string()
       }
   }

   fn print_val(valor: &str) {
      println!("Instrumento: {}", valor);
   }
}

// -- Struct with private an public values

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

// -- Enum pub -> All his members pub
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// -- Bringing a module into scope with use
fn TestScopeAbsolute() {
    use crate::sound::instrument;
    println!("In my absolute scope - {}",instrument::clarinet());
}

fn TestScopeRelative() {
    use self::sound::instrument;
    println!("In my relative scope - {}",instrument::clarinet());
}

fn main() {

    // -- Module with private an public 
    println!("Relativo - {}",sound::instrument::clarinet());
    println!("Absoluto - {}",crate::sound::instrument::clarinet());
    println!("");

    // -- Struct with private an public values
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);
    println!("");

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);

    // -- Enum pub -> All his members pub
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
    println!("");
 
    // -- Bringing a module into scope with use
    TestScopeAbsolute();
    TestScopeRelative();
    println!("");

}

// Used to create custom data types

// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct TuppleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

// Create some function associated with the Person struct
impl Person {
    // Construct
    fn new(first: &str, last: &str) -> Person {
        Person { first_name: first.to_string(), last_name: last.to_string() }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    
    // Name to tuple
    fn to_tuple(self) -> (String, String) { 
        (self.first_name, self.last_name)
    }
}


pub fn run () {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    // Mutating a value of the struct
    c.red = 200;
    println!("rgb value {}, {}, {}", c.red, c.green, c.blue);

    let mut tc = TuppleColor(255, 0, 0);
    tc.0 = 50;
    println!("rgb value {}, {}, {}", tc.0, tc.1, tc.2);
    let mut p = Person::new("John", "Doe");
    println!("full name: {}", p.full_name());
    p.set_last_name("Foo");
    println!("Person first name: {} and last name {}", p.first_name, p.last_name);
    println!("Person to tuple {:?}", p.to_tuple());
}

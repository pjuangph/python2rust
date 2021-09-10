use inline_python::python;

// This is better for reading 
fn main() {
    let who = "world";
    let n = 5;
    python! {
        for i in range('n):
            print(i, "Hello", 'who) // This is actual rust code so no single quotes 
        print("Goodbye") // need to adhere to rust standards 
    }
}

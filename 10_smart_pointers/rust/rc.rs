use std::rc::Rc;

// unique_ptr => boxed::Box<T>  // only one instance can exist
// shared_ptr => rc::Rc<T>      // reference count goes to 0 then deallocated
// weak_ptr => rc::Weak<T>      // This is an observer type of pointer. It can be deleted from some place but still exists in others hence (weak) 
fn main() {
    let rc_examples = "Rc examples".to_string();
    // new scope
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);    // Creates a stronger pointer to string
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        // one more scope
        {
            let rc_b: Rc<String> = Rc::clone(&rc_a); // Strong pointer to another strong pointer
            println!("Ref Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Ref Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
        // This is fine because there's no circular references 
    }
}

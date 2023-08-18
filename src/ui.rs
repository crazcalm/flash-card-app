use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.txt")] // using the template in this path, relative
                                // to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

pub fn helloworld(name: &str) -> String {
    let hello = HelloTemplate { name }; // instantiate your struct
 // then render it.
    hello.render().unwrap()
}

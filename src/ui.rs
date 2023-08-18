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

#[derive(Template)] // this will generate the code...
#[template(path = "flash_card.txt")] // using the template in this path, relative
struct FlashCardTemplate<'a> {
    side: &'a str,
    text: &'a str,
    seen: usize,
    total: usize,
}

pub fn flashcard(side: &str, text: &str, seen: usize, total: usize) -> String {
    let flashcard_template = FlashCardTemplate { side, text, seen, total }; 
    flashcard_template.render().unwrap()
}

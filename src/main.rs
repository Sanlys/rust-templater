use template_macro::template_macro;

fn main() {
    let title = "Header";
    let text = "
        This is a block of text bla bla Should add newlines
    ";
    let output = template_macro! {
        <div>
            <h1>{{title}}</h1>
            <p>{{text}}</p>
        </div>
    };
    println!("{output}")
}

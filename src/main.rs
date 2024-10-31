use template_macro::template_macro;

fn main() {
    let title = "Header";
    let output = template_macro!("test.html");
    println!("{output}");
}

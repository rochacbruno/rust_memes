use std::fs;
use std::path::Path;

fn render(image_path: &str) -> String {
    let s: String = format!(
        "https://github.com/rochacbruno/rust_memes/blob/master/{image_path}
![https://raw.githubusercontent.com/rochacbruno/rust_memes/master/{image_path}](https://raw.githubusercontent.com/rochacbruno/rust_memes/master/{image_path})
---",
        image_path = image_path
    );
    s.trim().to_string()
}

fn main() {
    let path = Path::new("img");
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        println!("{}", render(entry.path().to_str().expect("a")));
    }
}

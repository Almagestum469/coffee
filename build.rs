#[cfg(windows)]
fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/coffee.ico");
        res.compile().unwrap();
    }
}

#[cfg(not(windows))]
fn main() {}

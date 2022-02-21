use libfetch::{Info, platform::Platform};

fn main() {
    let os = Info {};

    let os_name = os.name();
    let kernel = os.kernel();

    println!("os ~> {}", os_name.unwrap_or(String::from("unknown")));
    println!("kernel ~> {}", kernel.unwrap_or(String::from("unknown")));
}

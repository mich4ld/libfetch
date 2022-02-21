use fetch_os::{linux, platform::Platform};

fn main() {
    let os = linux::Linux {};

    let os_name = os.name();
    let kernel = os.kernel();

    println!("os ~> {}", os_name.unwrap_or(String::from("unknown")));
    println!("kernel ~> {}", kernel.unwrap_or(String::from("unknown")));
}

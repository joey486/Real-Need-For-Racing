extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_resource_file("icon.rc");
        match res.compile() {
            Ok(_) => println!("Resource compilation succeeded"),
            Err(e) => eprintln!("Resource compilation failed: {:?}", e),
        }
    }
}
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_resource_file("icon.rc");
        res.compile().unwrap();
    }
}
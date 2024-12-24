use std::env;

fn main() {
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .derive_default(true)
        // I think this will recompile for included the header files as well
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("/usr/include/linux/uhid.h")
        .generate()
        .expect("Unable to generate bindings");

    let path = env::current_dir().expect("Couldn't get CWD");
    bindings
        .write_to_file(path.join("src/uhid.rs"))
        .expect("Couldn't write bindings");
}

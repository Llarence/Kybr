use std::env;

fn compile_header(header: &str, out_name: &str) {
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .derive_default(true)
        // I think this will recompile for included the header files as well
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("/usr/include/".to_owned() + header)
        .generate()
        .expect("Unable to generate bindings");

    let path = env::current_dir().expect("Couldn't get CWD");
    bindings
        .write_to_file(path.join("src/".to_owned() + out_name))
        .expect("Couldn't write bindings");
}

fn main() {
    compile_header("linux/uhid.h", "uhid.rs");
    compile_header("linux/input.h", "input.rs");
}

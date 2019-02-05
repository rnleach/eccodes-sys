use std::{env, path::PathBuf};

fn main() {
    // Tell cargo to link to the eccodes library, which needs to be installed on the system
    // already.
    pkg_config::Config::new()
        .atleast_version("2.10")
        .probe("eccodes")
        .expect("failed to find eccodes library; eccodes or pkg-config not properly installed.");

    //
    // Find the header file path
    //
    let header_path = pkg_config::get_variable("eccodes", "cflags")
        // Convert to Option so we can use nth method in next step
        .ok()
        // Take the first value returned (for some reason it returns the same -I path twice)
        .and_then(|var_string| {
            var_string
                .split_whitespace()
                .nth(0)
                .map(ToString::to_string)
        })
        // Make a PathBuf, but drop the leading -I
        .map(|var_string| PathBuf::from(var_string[2..].to_owned()))
        // Get out of Result/Option land
        .expect("unable to find eccodes header file")
        // Append the file name
        .join("eccodes.h");

    //
    // Create the bindings
    //
    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .rust_target(bindgen::RustTarget::Stable_1_33)
        .whitelist_function("codes_.*")
        .whitelist_type("codes_.*")
        .whitelist_var("CODES_.*")
        .parse_callbacks(Box::new(CustomParse))
        .blacklist_item("FILE")
        .blacklist_item("_IO.*")
        .blacklist_item("off_t")
        .blacklist_item("__off.*")
        .raw_line("use libc::{FILE, off_t};")
        .rustified_enum("ProductKind")
        .derive_copy(false)
        .derive_debug(false)
        .impl_debug(false)
        .derive_partialord(false)
        .impl_partialeq(false)
        .generate_comments(false)
        .ctypes_prefix("libc")
        .rustfmt_bindings(true)
        .generate()
        .expect("unable to generate bindings from eccodes.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("no OUT_DIR environment variable"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Uncomment the lines below to save the generated code in an easy to find location for
    // inspection and debugging.

    // let out_path = PathBuf::from("./generated_code_eccodes.rs");
    // bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}

#[derive(Debug)]
struct CustomParse;

impl bindgen::callbacks::ParseCallbacks for CustomParse {
    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        use bindgen::callbacks::IntKind::*;

        let val = if name.starts_with("CODES_KEYS_ITERATOR") {
            ULong
        } else {
            Int
        };

        Some(val)
    }
}

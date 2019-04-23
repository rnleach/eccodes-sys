use flate2::read::GzDecoder;
use std::{
    env,
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};
use tar::Archive;

const VERSION: &str = include_str!("version"); // Requires x.y.z, all x, y, and z!
const EXTRACTED_LIB: &str = concat!("eccodes-", include_str!("version"), "-Source");
const ARCHIVED_LIB: &str = concat!("eccodes-", include_str!("version"), "-Source.tar.gz");

fn main() {
    let home_dir_install = dirs::home_dir().unwrap().join("usr");
    let pc_dir = format!("{}/lib/pkgconfig/", home_dir_install.to_string_lossy());
    std::env::set_var("PKG_CONFIG_PATH", pc_dir);

    // Check to see if the library is installed, and install it if needed.
    let lib = match pkg_config::Config::new()
        .atleast_version(VERSION)
        .probe("eccodes")
    {
        // Use the lib!
        Ok(lib) => lib,
        // Try to install the library from source
        Err(pkg_config::Error::Failure {
            command: ref _command,
            output: ref _output,
        }) => {
            eprintln!("eccodes package not found, attempting to install...");
            install_eccodes_c_libs(home_dir_install).expect("FATAL ERROR INSTALLING ECCODES C LIB");
            pkg_config::Config::new()
                .atleast_version(VERSION)
                .probe("eccodes")
                .expect("UNABLE TO FIND ECCODES C LIB AFTER INSTALLING")
        }
        // Give up
        Err(err) => panic!("FATAL ERROR: {}", err),
    };

    let header_path = lib.include_paths[0]
        .join("eccodes.h")
        .canonicalize()
        .unwrap();

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
    // bindings
    //     .write_to_file(out_path)
    //     .expect("Couldn't write bindings!");
}

fn install_eccodes_c_libs<P: AsRef<Path>>(home_dir_install: P) -> Result<(), Box<Error>> {
    let mut arch_path = PathBuf::new();
    arch_path.push(".");
    arch_path.push("lib");
    let arch_path = arch_path.join(ARCHIVED_LIB);

    let tmp_dir = tempdir::TempDir::new("eccodes_install")?;
    let src_path = tmp_dir.as_ref().join(EXTRACTED_LIB);

    let home_dir_install: &Path = home_dir_install.as_ref();

    let tar_gz = File::open(arch_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(tmp_dir.as_ref())?;

    std::fs::create_dir_all(home_dir_install.join("bin"))?;
    std::fs::create_dir_all(home_dir_install.join("lib"))?;
    std::fs::create_dir_all(home_dir_install.join("include"))?;
    std::fs::create_dir_all(home_dir_install.join("share"))?;

    let _dst = cmake::Config::new(src_path)
        .define("ENABLE_FORTRAN", "OFF")
        .define("CMAKE_INSTALL_PREFIX", home_dir_install)
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    tmp_dir.close()?;

    Ok(())
}

#[derive(Debug)]
struct CustomParse;

impl bindgen::callbacks::ParseCallbacks for CustomParse {
    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        use bindgen::callbacks::IntKind::*;

        let val = if name.starts_with("CODES_KEYS_ITERATOR") {
            ULong
        } else if name == "CODES_MISSING_LONG" {
            Long
        } else {
            Int
        };

        Some(val)
    }
}

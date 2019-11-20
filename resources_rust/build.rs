use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=sourcetraildb");
    println!("cargo:rustc-link-search=../build/core");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Stable_1_33)
        // .rust_target(bindgen::RustTarget::Nightly)
        .enable_cxx_namespaces()
        .conservative_inline_namespaces()
        // .trust_clang_mangling(false)
        .clang_args(&["-x", "c++"])
        .clang_arg("-std=c++11")
        .derive_default(true)
        .derive_debug(true)
        .clang_arg("-I../core/include")
        .clang_arg("-I../external/cpp_sqlite/include")
        .header("wrapper.hpp")
        //
        .opaque_type("std::.*")
        .whitelist_type("sourcetrail::.*")
        .whitelist_function("sourcetrail::.*")
        //
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

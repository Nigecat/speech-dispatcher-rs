extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=speechd");
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .header("wrapper.h")
        .constified_enum_module("SPDConnectionMode")
        .constified_enum_module("SPDPriority")
        .constified_enum_module("SPDVoiceType")
        .constified_enum_module("SPDDataMode")
        .constified_enum_module("SPDNotification")
        .constified_enum_module("SPDPunctuation")
        .constified_enum_module("SPDCapitalLetters")
        .constified_enum_module("SPDSpelling")
        .use_core()
        .layout_tests(false)
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("speech_dispatcher_sys.rs"));
}

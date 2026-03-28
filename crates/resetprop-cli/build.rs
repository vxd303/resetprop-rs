use std::env;
use std::fs;
use std::path::PathBuf;

const OBF_KEY: u8 = 0x5A;

fn obfuscate(input: &str) -> Vec<u8> {
    input.bytes().map(|b| b ^ OBF_KEY).collect()
}

fn main() {
    println!("cargo:rerun-if-env-changed=RESETPROP_FRAMEWORK_SHA256");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    let out_file = out_dir.join("framework_gate.rs");

    let expected = env::var("RESETPROP_FRAMEWORK_SHA256").ok();
    let body = match expected {
        Some(value) => {
            let obf = obfuscate(value.trim());
            format!(
                "const OBF_EXPECTED_FRAMEWORK_SHA256: Option<&[u8]> = Some(&{:?});\n",
                obf
            )
        }
        None => "const OBF_EXPECTED_FRAMEWORK_SHA256: Option<&[u8]> = None;\n".to_string(),
    };

    fs::write(out_file, body).expect("failed to write framework_gate.rs");
}

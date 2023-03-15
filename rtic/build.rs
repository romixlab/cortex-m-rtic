use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    // These targets all have know support for the BASEPRI register.
    if target.starts_with("thumbv7m")
        | target.starts_with("thumbv7em")
        | target.starts_with("thumbv8m.main")
    {
        println!("cargo:rustc-cfg=feature=\"cortex-m-basepri\"");
    } else if target.starts_with("thumbv6m") | target.starts_with("thumbv8m.base") {
        println!("cargo:rustc-cfg=feature=\"cortex-m-source-masking\"");
    } else if target.starts_with("riscv32i") {
        panic!("No RISC-V support yet.");

        // TODO: Add feature here for risc-v targets
        // println!("cargo:rustc-cfg=feature=\"riscv\"");
    } else if target.starts_with("thumb") || target.starts_with("riscv32") {
        panic!("Unknown target '{target}'. Need to update logic in build.rs.");
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn main() {
    nasm_rs::compile_library_args(
        "mss32jumptable",
        &["asm/mss32jumptable.asm"],
        &["-f", "win32", "--prefix", "_"],
    )
    .expect("Failed to assemble NASM sources");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=asm/mss32jumptable.asm");
    println!("cargo:rustc-link-lib=static=mss32jumptable");

    println!("cargo:rerun-if-changed=resource.rc");
    println!("cargo:rerun-if-changed=manifest.xml");

    embed_resource::compile("resource.rc", embed_resource::NONE)
        .manifest_required()
        .unwrap();
}

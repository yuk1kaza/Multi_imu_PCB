fn main() {
    let fusion_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .and_then(|p| p.parent())
        .unwrap()
        .join("contrib")
        .join("fusion");

    println!("cargo:rerun-if-changed={}", fusion_dir.display());

    cc::Build::new()
        .include(&fusion_dir)
        .file(fusion_dir.join("FusionAhrs.c"))
        .file(fusion_dir.join("FusionCompass.c"))
        .file(fusion_dir.join("FusionOffset.c"))
        .compile("fusion");
}

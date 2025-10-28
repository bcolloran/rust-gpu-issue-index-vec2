fn main() {
    build_spirv_kernel();
}

fn build_spirv_kernel() {
    use spirv_builder::SpirvBuilder;
    use std::path::PathBuf;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let kernels_path = PathBuf::from(manifest_dir).join("kernels");

    let result = SpirvBuilder::new(kernels_path, "spirv-unknown-vulkan1.4")
        .scalar_block_layout(true)
        .print_metadata(spirv_builder::MetadataPrintout::Full)
        .build()
        .unwrap();

    println!(
        "cargo:warning=building SPIRV to: {}",
        result.module.unwrap_single().display()
    );
    println!(
        "cargo:warning=SPIRV entry points: {}",
        result.entry_points.join(", ")
    );
    println!(
        "cargo:rustc-env=SHADERS_SPV_PATH={}",
        result.module.unwrap_single().display()
    );
}

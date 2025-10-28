fn main() {
    println!("cargo:rerun-if-changed=kernel/src");
    build_spirv_kernel();
}

fn build_spirv_kernel() {
    use spirv_builder::SpirvBuilder;
    use std::path::PathBuf;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let kernels_path = PathBuf::from(manifest_dir).join("kernels");

    let builder =
        SpirvBuilder::new(kernels_path, "spirv-unknown-vulkan1.4").scalar_block_layout(true);

    #[cfg(feature = "variable_pointers")]
    let builder = builder.capability(spirv_builder::Capability::VariablePointers);
    #[cfg(feature = "variable_pointers_storage_buffer")]
    let builder = builder.capability(spirv_builder::Capability::VariablePointersStorageBuffer);

    let result = builder
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

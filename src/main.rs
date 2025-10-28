use spirv_std::glam::Vec2;

mod vulkano_runner;

pub const WORKGROUP_SIZE: usize = 64;
pub const KERNEL_SPIRV: &[u8] = include_bytes!(env!("SHADERS_SPV_PATH"));
pub const KERNEL_ENTRY_POINT: &str = "index_vec2";

fn main() {
    println!("Spir-V shader byte length: {}", KERNEL_SPIRV.len());

    let runner = vulkano_runner::VulkanoRunner::new().unwrap_or_else(|e| {
        panic!("Failed to create VulkanoRunner: {:?}", e);
    });
    let len = 128;
    let mut data = vec![Vec2::new(1.0, 2.0); len];

    println!(
        "Data before running compute shader (first 10 elts): {:?}",
        &data[..10]
    );

    runner.run_pass(&mut data).unwrap();

    println!(
        "Data after running compute shader (first 10 elts): {:?}",
        &data[..10]
    );
}

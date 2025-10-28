pub const SHADERS_SPIRV: &[u8] = include_bytes!(env!("SHADERS_SPV_PATH"));

fn main() {
    println!("SpIR-V shader byte length: {}", SHADERS_SPIRV.len());
}

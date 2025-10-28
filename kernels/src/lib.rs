#![no_std]
use spirv_std::{
    glam::{UVec3, Vec2},
    spirv,
};

#[spirv(compute(threads(64)))]
pub fn index_vec2(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] a: &mut [Vec2],
) {
    let i = id.x as usize;
    let val = &mut a[i];

    // This works fine:
    val.x += i as f32;
    val.y += i as f32 * 2.0;

    // this compiles and runs with:
    // `cargo run --features variable_pointers variable_pointers_storage_buffer`
    let v0 = val[0];
    val[1] = v0 + 0.01;

    // this fails to compile with plain `cargo run`, which provides an error message about missing capabilities.
    //
    // However: if you enable those capabilities, the code compiles but segfaults at runtime.
    //
    // To get this to work correctly, you need to **comment out** the code below, then cargo run with the features enabled. That should run without error. **Then** uncomment the code below and run again with the features enabled, and it should work.
    for j in 0..2 {
        let v = val[j];
        val[j] = v + (i * (j + 1)) as f32 * 0.00001;
    }
}

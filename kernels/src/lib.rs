#![no_std]
use spirv_std::{
    glam::{UVec3, Vec2},
    spirv,
};

#[spirv(compute(threads(64)))]
pub fn index_vec2(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] a: &[Vec2],
) {
    let i = id.x as usize;
    let mut val = a[i];

    val.x += i as f32;
    val.y += i as f32 * 2.0;
    // this compiles fine:
    // let v0 = val[0];
    // val[1] = 10.0;

    // this fails to compile::
    // for j in 0..2 {
    //     let _v = val[j];
    // }
}

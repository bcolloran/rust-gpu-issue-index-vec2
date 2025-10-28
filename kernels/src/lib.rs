#![no_std]
use spirv_std::{
    glam::{UVec3, Vec2},
    spirv,
};

#[spirv(compute(threads(64)))]
pub fn index_vec_2(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] a: &[Vec2],
) {
    let i = id.x as usize;
    let val = a[i];

    // this compiles fine:
    val[0];

    // this fails to compile::
    for j in 0..2 {
        let _v = val[j];
    }
}

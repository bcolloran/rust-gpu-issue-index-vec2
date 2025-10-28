# Unable to compile rust-gpu shader using index vec2 with dynamic indexing

When trying to compile a rust-gpu shader that uses `Vec2` and attempts to index it dynamically (e.g., in a loop), the build fails with errors related to `OpPhi` and pointer capabilities.

Just `cargo build` to see the error below.

### kernel code

(In file `kernels/src/lib.rs`)

```rust
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
```


### error output
```
error: failed to run custom build command for `rust-gpu-issue-index-vec2 v0.1.0 (/data/code_projects/rust/rust-gpu-issue-index-vec2)`

Caused by:
  process didn't exit successfully: `/data/code_projects/rust/rust-gpu-issue-index-vec2/target/debug/build/rust-gpu-issue-index-vec2-db683b59b66f7e48/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=RUSTGPU_CODEGEN_ARGS
  cargo:rerun-if-env-changed=RUSTGPU_RUSTFLAGS
  cargo:rerun-if-env-changed=RUSTGPU_CARGOFLAGS

  --- stderr
     Compiling kernels v0.0.0 (/data/code_projects/rust/rust-gpu-issue-index-vec2/kernels)
  error: Using pointers with OpPhi requires capability VariablePointers or VariablePointersStorageBuffer
    |
    = note: module `/data/code_projects/rust/rust-gpu-issue-index-vec2/target/spirv-builder/spirv-unknown-vulkan1.4/release/deps/kernels.spv`

  warning: an unknown error occurred
    |
    = note: spirv-opt failed, leaving as unoptimized
    = note: module `/data/code_projects/rust/rust-gpu-issue-index-vec2/target/spirv-builder/spirv-unknown-vulkan1.4/release/deps/kernels.spv`

  error: error:0:0 - Using pointers with OpPhi requires capability VariablePointers or VariablePointersStorageBuffer
           %95 = OpPhi %_ptr_Function_float %93 %91 %94 %92
    |
    = note: spirv-val failed
    = note: module `/data/code_projects/rust/rust-gpu-issue-index-vec2/target/spirv-builder/spirv-unknown-vulkan1.4/release/deps/kernels.spv`

  warning: `kernels` (lib) generated 1 warning
  error: could not compile `kernels` (lib) due to 2 previous errors; 1 warning emitted

  thread 'main' panicked at build.rs:16:10:
  called `Result::unwrap()` on an `Err` value: BuildFailed
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```

## Environment
rustc 1.90.0-nightly (35f603652 2025-06-29)

System:
  Linux Mint 22.1 Xia base: Ubuntu 24.04 noble
  Kernel: 6.8.0-85-generic arch: x86_64

from `Cargo.lock`:
[[package]]
name = "rustc_codegen_spirv"
version = "0.9.0"
source = "git+https://github.com/rust-gpu/rust-gpu?branch=main#e767f24f2565baf1a71bbaf84d453d181cab2417"

_(main as of 2025-10-27)_


## Note:
This is just a minimal repro for the issue; in a bigger project, I'm able to build successfully when I add `.capability(Capability::VariablePointers)`, but then I get a segfault when trying to run the shader.
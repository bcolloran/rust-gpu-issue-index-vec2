# Weird segfault when indexing Vec2 dynamically in rust-gpu shader

## Initial error
When trying to compile a rust-gpu shader that uses `Vec2` and attempts to index it dynamically (e.g., in a loop), the build fails with errors related to `OpPhi` and pointer capabilities. This is expected, and you get the error below, which mentions missing capabilities.

Do `cargo run` to see the error below:

### error output
```shell
error: failed to run custom build command for `rust-gpu-issue-index-vec2 v0.1.0 (/data/code_projects/rust/rust-gpu-issue-index-vec2)`

Caused by:
  process didn't exit successfully: `/data/code_projects/rust/rust-gpu-issue-index-vec2/target/debug/build/rust-gpu-issue-index-vec2-bd3276fa397dd4b3/build-script-build` (exit status: 101)
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
           %62 = OpPhi %_ptr_StorageBuffer_float %60 %58 %61 %59
    |
    = note: spirv-val failed
    = note: module `/data/code_projects/rust/rust-gpu-issue-index-vec2/target/spirv-builder/spirv-unknown-vulkan1.4/release/deps/kernels.spv`

  warning: `kernels` (lib) generated 1 warning
  error: could not compile `kernels` (lib) due to 2 previous errors; 1 warning emitted

  thread 'main' panicked at build.rs:23:10:
  called `Result::unwrap()` on an `Err` value: BuildFailed
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Adding capabilities

Adding the capabilities mentioned indeed fixes the build error, but then results in an **intermittent** segfault when trying to run the shader.

Reproduce with `cargo run --features variable_pointers variable_pointers_storage_buffer` to see the segfault:

### segfault output
```shell
$ cargo run --features variable_pointers variable_pointers_storage_buffer
warning: rust-gpu-issue-index-vec2@0.1.0: building SPIRV to: /data/code_projects/rust/rust-gpu-issue-index-vec2/target/spirv-builder/spirv-unknown-vulkan1.4/release/deps/kernels.spv
warning: rust-gpu-issue-index-vec2@0.1.0: SPIRV entry points: index_vec2
   Compiling rust-gpu-issue-index-vec2 v0.1.0 (/data/code_projects/rust/rust-gpu-issue-index-vec2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/rust-gpu-issue-index-vec2 variable_pointers_storage_buffer`
Spir-V shader byte length: 227128
Segmentation fault (core dumped)
```

## Using `println!("cargo:rerun-if-changed=kernel/src");` in `build.rs`
The segfault seems to be related to caching of the built SPIR-V shader. To ensure that the shader is rebuilt every time, add the following line to `build.rs` seems to resolve it???:

```rust
println!("cargo:rerun-if-changed=kernel/src");
```

Ouput from a terminal session showing the before and after of adding that line available in `teminal_session.txt`.



# In my real project
In my real project, which is set up similarly but is of course more complex, I'm hitting a similar but slightly different behavior. Whenever I make changes to the shader code, I get a segfault if I use `cargo run --features variable_pointers variable_pointers_storage_buffer`. If I then run a plain `cargo run`, I get the expected error about missing capabilities. Then, if I run `cargo run --features variable_pointers variable_pointers_storage_buffer` again, (most of the time) it works fine without segfaulting.

But having to compile twice every time I change the shader is tedious.
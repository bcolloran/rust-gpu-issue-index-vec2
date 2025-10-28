# Getting started
Before starting any implementation, the agent should read the entire specification provided by the user. The agent should make sure to understand the requirements and constraints of the task. The agent should come up with a plan for how to implement the task, breaking it down into smaller sub-tasks if necessary.

The agent should make note of any apparent misspecifications, ambiguities in the spec, or edge cases or surprises that come up during the implementation, and report those to the user in the post-implementation summary and in the PR description.

In most cases, the agent should make a best-effort attempt to implement the task, and inform the user of any uncertainties or issues that arise during the implementation. However, if the agent is *very* confident that the task is untenable, or if there are any *deep* uncertainties about the task, the agent should inform the user and ask for guidance before starting any implementation.

The agent must always read the root `README.md` for large scale context. Depending on the part of the code the agent is working on, there may be relevant topic-specific READMEs (e.g., in `src/graphics/README.md`), and the agent should read those as well. 


# Tests
**Before making any changes to the code, the agent must run all tests to ensure that they pass. If any tests fail, the agent must immediately halt all work and inform the user.**


## Testing *user-written* implementation code
- When writing tests, on code that has been written before the agent's current task, the agent must first analyze the existing code within the context of the overall codebase, along with any comments or descriptions that explain its purpose and functionality.
- The users code may contain errors or inconsistencies! The agents task is to be BRUTALLY CRITICAL of existing code, and to attempt to write tests that will poke holes in the code's correctness and reveal any bugs or issues.
- The agent should identify and address these issues in the tests, ensuring that the tests accurately reflect the intended behavior of the code. It's ok if the tests reveal bugs or issues in the existing code; the goal is to ensure that the code behaves correctly and reliably. Failing tests are ok, and should be reported back to the user for fixing.
- If the agent identifies any issues or inconsistencies in the existing code while writing tests, it should document these findings and provide suggestions for how to address them. However, the agent **must not** modify the existing code unless explicitly instructed to do so by the user.

## Testing *agent-written* implementation code
- When writing tests on code that the agent has written itself during the current task, the agent should ensure that the tests are comprehensive and cover all relevant scenarios for the newly written code.
- The agent should write tests that cover the expected behavior of the newly written code, including edge cases and potential failure modes.
- When writing tests for its own code, the agen may iteratively refine both the code and the tests to ensure that they are correct and reliable.

## Running tests
The agent should always run `cargo test` in the workspace root to run all tests in the workspace, not just the tests in the current crate. Do not pass additional flags to `cargo test` unless specifically requested by the user.

## Test file organization
Try to ensure that the file organization for tests matches that of the implementation code, e.g., if `TraitX` for `TypeY` is implemented in the file `type_y.rs`, put tests in `tests/type_y.rs`. If there are many tests for a single file, consider putting them in a subdirectory, e.g., `tests/type_y/mod.rs` and `tests/type_y/other_tests.rs`.

## Multiple cases in one test
When it makes sense to repeatedly test a single function on multiple *input* cases, use a `#[test_case(data; "data case description")]` attribute on a test to specify the data cases. This allows the test to be run multiple times with different inputs, and will report each case separately in the test results.

This is "DRY"er than writing a separate test function for each case, and cleaner than putting multiple assertion statements in a single test function that loops over the data cases.

For example, we have this in the file `easy_hash/tests/test_utils.rs`:
```rust
#[test_case(0 ; "0u64")]
#[test_case(1 ; "1u64")]
#[test_case(u32::MAX as u64 ; "u32::MAX as u64")]
#[test_case(u64::MAX ; "u64::MAX")]
#[test_case(u64::MAX - 1 ; "u64::MAX - 1")]
#[test_case(0x1234_5678_9abc_def0 ; "0x1234_5678_9abc_def0")]
fn test_split_u64_roundtrip(val: u64) {
    let parts = split_u64(val);
    assert_eq!(join_u32s(parts[0], parts[1]), val);
}
```

# Implemetation practices
## Readability and comments **IMPORTANT**
**The user is a novice in shader programming and graphics programming. The agent should take extra care to write clear, well-documented code that is easy for a beginner to understand.** 

This project is intended to be educational, so clarity and simplicity are more important than performance optimizations or advanced techniques. (`TODO` Comments may be added describing future improvements, but the agent should avoid adding complex or advanced features that may confuse a beginner.)

The agent should strive to write code that is easy to read and understand. This includes using clear and descriptive variable and function names, as well as adding comments where necessary to explain complex logic or design. Functions and methods should always have doc comments explaining their purpose, parameters, and return values.

## File size recommendation
If you try to limit your file sizes to a couple hundred lines and make sure that each file is focused on a smaller unit of fumctionality. Files that are longer than about 200-300 lines is a code smell; there is no hard rule against this, but it may indicate that the code could benefit from being broken into smaller modular parts. 

Also sticking with short files introduces less risk of one big file being corrupted during editing.

## Updating documentation
Whenever you make changes to the codebase, you should also update any relevant documentation to reflect those changes. This includes updating comments in the code, as well as updating any external documentation such as README files.





# "Removing" files, "corrupted" files, and file size limits
Sometimes an agent will get stuck in a loop of thinking that a file is "corrupted", and will attempt to remove and rewrite the file over and over. 

To avoid this: rather than removing existing files, the agent should start fresh in a new file with a similar name. For example, if you think `some_file.rs` has become corrupted, create a new file `some_file_1.rs` (and so on in sequence), and just point all the modules that had used `some_file.rs` at the newest version.

Leave the old versions on disk. this will allow me to evaluate what went wrong.

# Commands
The agent may run cargo commands to build and test the project, but should not run any other commands.

## Pre-approved cargo commands
The agent is strongly encouraged to run the following cargo commands as needed:

`cargo test --workspace --verbose | head -60`
`cargo test --workspace --verbose | tail -60`
`cargo check --lib 2>&1 | head -60`
`cargo check --lib 2>&1 | tail -60`
**BE SURE TO ALWAYS USE _60_ LINES OF OUTPUT WHEN USING HEAD/TAIL!!!** 60 lines has been pre-approved, other values have not and will require user approval, which will slow down progress!!!

## FORBIDDEN COMMANDS
**THIS IS IMPORTANT**: The agent may **NEVER** run any commands that modify files on disk such as `rm`, `sed`, `mv`, `cp`, etc.

# Web access
The agent may request any pages from github.com. Accessing raw files via `https://github.com/.../raw/refs/...` is advised (or whatever is currently the best mechanism to get raw file contents).

Also highly recommend checking e.g. crates.io, and docs.rs for reference material.


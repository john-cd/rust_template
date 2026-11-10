// Move the rendered crates only after template files and their include patterns have been resolved.
let cli_folder_name = variable::get("cli_folder_name");
let lib_folder_name = variable::get("lib_folder_name");

if cli_folder_name != "my_cli" {
    file::rename("crates/my_cli", "crates/" + cli_folder_name);
}
if lib_folder_name != "my_lib" {
    file::rename("crates/my_lib", "crates/" + lib_folder_name);
}

// Generator settings and hooks are template implementation details, not project setup files.
file::delete("cargo-generate.toml");
file::delete("scripts/cargo-generate-pre.rhai");
file::delete("scripts/cargo-generate-post.rhai");

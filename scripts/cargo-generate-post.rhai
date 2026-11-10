// Move the rendered crates only after template files and their include patterns have been resolved.
let cli_folder_name = variable::get("cli_folder_name");
let lib_folder_name = variable::get("lib_folder_name");

// Keep the checked-in defaults in place; rename only folders the user customized.
if cli_folder_name != "my_cli" {
    file::rename("crates/my_cli", "crates/" + cli_folder_name);
}
if lib_folder_name != "my_lib" {
    file::rename("crates/my_lib", "crates/" + lib_folder_name);
}

// The generated workspace is a project, not a reusable template; remove its generator config and hooks.
file::delete("cargo-generate.toml");
file::delete("scripts/cargo-generate-pre.rhai");
file::delete("scripts/cargo-generate-post.rhai");

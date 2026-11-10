// Resolve package-name defaults and reject workspace name collisions before rendering.
let project_name = variable::get("project-name");
let cli_package_name = variable::get("cli_package_name");
let lib_package_name = variable::get("lib_package_name");

if cli_package_name == "auto" {
    variable::set("cli_package_name", project_name);
}
if lib_package_name == "auto" {
    variable::set("lib_package_name", project_name + "_lib");
}

let cli_folder_name = variable::get("cli_folder_name");
let lib_folder_name = variable::get("lib_folder_name");
if cli_folder_name == lib_folder_name {
    abort("CLI and library folders must have different names");
}
if variable::get("cli_package_name") == variable::get("lib_package_name") {
    abort("CLI and library packages must have different names");
}

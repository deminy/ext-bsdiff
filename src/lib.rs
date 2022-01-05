use ext_php_rs::prelude::*;

/// Diff an "old" and a "new" file, returning a patch.
///
/// @param string $old_file   The old file.
/// @param string $new_file   The new file.
/// @param string $patch_file The patch file that will be created.
/// @return bool Returns TRUE if succeeds, otherwise FALSE.
#[php_function]
pub fn bsdiff_diff(old_file: &str, new_file: &str, patch_file: &str) -> bool {
    let old = match std::fs::read(old_file) {
        Ok(old) => old,
        Err(_e) => return false,
    };
    let new = match std::fs::read(new_file) {
        Ok(new) => new,
        Err(_e) => return false,
    };
    let mut patch = Vec::new();

    match bsdiff::diff::diff(&old, &new, &mut patch) {
        Err(_e) => return false,
        _ => (),
    };
    match std::fs::write(patch_file, &patch) {
        Err(_e) => return false,
        _ => return true,
    };
}

/// Apply a patch to an "old" file, returning the "new" file.
///
/// @param string $old_file   The old file.
/// @param string $patch_file The patch file.
/// @param string $new_file   The new file that will be created.
/// @return bool Returns TRUE if succeeds, otherwise FALSE.
#[php_function]
pub fn bsdiff_patch(old_file: &str, patch_file: &str, new_file: &str) -> bool {
    let old = match std::fs::read(old_file) {
        Ok(old) => old,
        Err(_e) => return false,
    };
    let patch = match std::fs::read(patch_file) {
        Ok(patch) => patch,
        Err(_e) => return false,
    };
    let mut new = Vec::new();

    match bsdiff::patch::patch(&old, &mut patch.as_slice(), &mut new) {
        Err(_e) => return false,
        _ => (),
    };
    match std::fs::write(new_file, &new) {
        Err(_e) => return false,
        _ => return true,
    };
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

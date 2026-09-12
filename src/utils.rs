use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn is_file_in_dir(path: &str, filename: &str) -> bool {
    fs::read_dir(path)
        .into_iter()
        .flatten()
        .flatten()
        .any(|entry| entry.file_name() == filename)
}

pub fn can_exec(file: &Path) -> bool {
    fs::metadata(file)
        .ok()
        .filter(|meta| meta.is_file())
        .map(|meta| meta.permissions().mode())
        // 0o100  user
        // 0o010  group
        // 0o001  other
        .is_some_and(|mode| mode & 0o111 != 0)
}

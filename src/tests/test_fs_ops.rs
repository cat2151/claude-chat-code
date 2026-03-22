#[cfg(test)]
mod fs_ops_tests {
    use crate::fs::ops::{ensure_base_dirs, touch_src_files};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn ensure_base_dirs_creates_all_subdirs() {
        let dir = tempdir().unwrap();
        ensure_base_dirs(dir.path()).unwrap();
        for sub in &["archives", "project", "backup"] {
            assert!(dir.path().join(sub).exists(), "{} が存在しない", sub);
        }
    }

    #[test]
    fn ensure_base_dirs_is_idempotent() {
        let dir = tempdir().unwrap();
        ensure_base_dirs(dir.path()).unwrap();
        ensure_base_dirs(dir.path()).unwrap();
    }

    #[test]
    fn touch_src_files_returns_zero_for_nonexistent_dir() {
        assert_eq!(touch_src_files(std::path::Path::new("/nonexistent/src")).unwrap(), 0);
    }

    #[test]
    fn touch_src_files_updates_all_files() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("a.rs"), b"").unwrap();
        fs::write(src.join("b.rs"), b"").unwrap();
        assert_eq!(touch_src_files(&src).unwrap(), 2);
    }
}

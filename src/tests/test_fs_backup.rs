#[cfg(test)]
mod fs_backup_tests {
    use crate::fs::backup::{backup_project, next_backup_name};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn backup_project_copies_files_and_ignores_target() {
        let base = tempdir().unwrap();
        let src = base.path().join("project");
        let dst = base.path().join("backup").join("backup00");

        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("Cargo.toml"), b"[package]").unwrap();
        fs::create_dir_all(src.join("src")).unwrap();
        fs::write(src.join("src").join("main.rs"), b"fn main(){}").unwrap();
        fs::create_dir_all(src.join("target").join("debug")).unwrap();
        fs::write(src.join("target").join("debug").join("app.exe"), b"binary").unwrap();

        backup_project(&src, &dst).unwrap();

        assert!(dst.join("Cargo.toml").exists());
        assert!(dst.join("src").join("main.rs").exists());
        assert!(!dst.join("target").exists(), "target/ はコピーされるべきではない");
    }

    #[test]
    fn backup_project_succeeds_when_src_does_not_exist() {
        let base = tempdir().unwrap();
        let src = base.path().join("nonexistent_project");
        let dst = base.path().join("backup00");
        backup_project(&src, &dst).unwrap();
        assert!(dst.exists());
    }

    #[test]
    fn next_backup_name_starts_with_prefix() {
        assert!(next_backup_name().starts_with("backup_"));
    }

    #[test]
    fn next_backup_name_has_correct_length() {
        // backup_YYYYMMDD_HHMMSS = 7 + 8 + 1 + 6 = 22
        assert_eq!(next_backup_name().len(), 22);
    }

    #[test]
    fn next_backup_name_is_unique_over_time() {
        let a = next_backup_name();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let b = next_backup_name();
        assert_ne!(a, b);
    }
}

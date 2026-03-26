#[cfg(test)]
mod paths_tests {
    use crate::paths::{archives_dir, backup_root, project_dir, src_dir};
    use std::path::Path;

    fn fake_work() -> std::path::PathBuf {
        Path::new(r"C:\Users\test\AppData\Local\claude-chat-code\work").to_path_buf()
    }

    #[test]
    fn archives_dir_is_under_work() {
        let w = fake_work();
        let a = archives_dir(&w);
        assert!(a.starts_with(&w));
        assert!(a.to_string_lossy().contains("archives"));
    }

    #[test]
    fn project_dir_is_under_work() {
        assert!(project_dir(&fake_work()).starts_with(fake_work()));
    }

    #[test]
    fn src_dir_is_under_project() {
        let w = fake_work();
        assert!(src_dir(&w).starts_with(project_dir(&w)));
    }

    #[test]
    fn backup_root_is_under_work() {
        assert!(backup_root(&fake_work()).starts_with(fake_work()));
    }
}

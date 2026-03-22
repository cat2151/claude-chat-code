#[cfg(test)]
mod watcher_tests {
    use crate::app::FileEntry;
    use crate::watcher::{find_latest_zip, has_changed, list_files};
    use chrono::Local;
    use std::time::{Duration, SystemTime};
    use tempfile::tempdir;

    fn make_entry(name: &str) -> FileEntry {
        FileEntry { name: name.to_string(), modified: Local::now() }
    }

    #[test]
    fn find_latest_zip_returns_first_zip_in_list() {
        let files = vec![make_entry("readme.txt"), make_entry("build.zip"), make_entry("old.zip")];
        assert_eq!(find_latest_zip(&files), Some("build.zip".to_string()));
    }

    #[test]
    fn find_latest_zip_returns_none_if_no_zip() {
        let files = vec![make_entry("a.txt"), make_entry("b.rs")];
        assert_eq!(find_latest_zip(&files), None);
    }

    #[test]
    fn find_latest_zip_case_insensitive() {
        let files = vec![make_entry("UPPER.ZIP")];
        assert_eq!(find_latest_zip(&files), Some("UPPER.ZIP".to_string()));
    }

    #[test]
    fn has_changed_true_when_prev_none_and_current_some() {
        assert!(has_changed(None, Some(SystemTime::now())));
    }

    #[test]
    fn has_changed_false_when_both_none() {
        assert!(!has_changed(None, None));
    }

    #[test]
    fn has_changed_false_when_same_time() {
        let t = SystemTime::now();
        assert!(!has_changed(Some(t), Some(t)));
    }

    #[test]
    fn has_changed_true_when_time_differs() {
        let t1 = SystemTime::now();
        let t2 = t1 + Duration::from_secs(1);
        assert!(has_changed(Some(t1), Some(t2)));
    }

    #[test]
    fn list_files_returns_only_files_not_dirs() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("sample.txt"), b"hello").unwrap();
        std::fs::create_dir(dir.path().join("subdir")).unwrap();
        let entries = list_files(dir.path());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "sample.txt");
    }

    #[test]
    fn list_files_sorted_newest_first() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("older.txt"), b"a").unwrap();
        std::thread::sleep(Duration::from_millis(10));
        std::fs::write(dir.path().join("newer.txt"), b"b").unwrap();
        let entries = list_files(dir.path());
        assert_eq!(entries[0].name, "newer.txt", "新しいファイルが先頭のはずだ");
    }

    #[test]
    fn list_files_returns_empty_for_nonexistent_dir() {
        assert!(list_files(std::path::Path::new("/nonexistent/path/xyz")).is_empty());
    }
}

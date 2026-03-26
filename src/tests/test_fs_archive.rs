#[cfg(test)]
mod fs_archive_tests {
    use crate::fs::archive::move_zip;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn move_zip_always_adds_timestamp_suffix() {
        let tmp = tempdir().unwrap();
        let desktop = tmp.path().join("desktop");
        let base = tmp.path().join("base");
        fs::create_dir_all(&desktop).unwrap();
        fs::create_dir_all(base.join("archives")).unwrap();
        fs::write(desktop.join("foo.zip"), b"zip").unwrap();

        let result = move_zip(&desktop, &base, "foo.zip").unwrap();

        // 常にタイムスタンプ付き → foo.zip という名前にはならない
        let name = result.file_name().unwrap().to_string_lossy().to_string();
        assert!(
            name.starts_with("foo_"),
            "タイムスタンププレフィックスがあるべきだ: {}",
            name
        );
        assert!(name.ends_with(".zip"));
        assert!(!desktop.join("foo.zip").exists());
    }

    #[test]
    fn move_zip_second_call_does_not_overwrite_first() {
        let tmp = tempdir().unwrap();
        let desktop = tmp.path().join("desktop");
        let base = tmp.path().join("base");
        fs::create_dir_all(&desktop).unwrap();
        fs::create_dir_all(base.join("archives")).unwrap();

        // 1回目
        fs::write(desktop.join("foo.zip"), b"first").unwrap();
        move_zip(&desktop, &base, "foo.zip").unwrap();

        // 2回目（1秒待って別タイムスタンプを確保）
        std::thread::sleep(std::time::Duration::from_secs(1));
        fs::write(desktop.join("foo.zip"), b"second").unwrap();
        move_zip(&desktop, &base, "foo.zip").unwrap();

        let entries: Vec<_> = fs::read_dir(base.join("archives"))
            .unwrap()
            .flatten()
            .collect();
        assert_eq!(entries.len(), 2, "archives に2ファイルあるべきだ");
    }
}

#[cfg(test)]
mod fs_extract_tests {
    use crate::fs::extract::extract_zip;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    fn make_zip(zip_path: &std::path::Path, entries: &[(&str, &[u8])]) {
        let file = fs::File::create(zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default();
        for (name, content) in entries {
            if name.ends_with('/') {
                zip.add_directory(*name, options).unwrap();
            } else {
                zip.start_file(*name, options).unwrap();
                zip.write_all(content).unwrap();
            }
        }
        zip.finish().unwrap();
    }

    #[test]
    fn strips_single_top_level_dir() {
        let tmp = tempdir().unwrap();
        let zip_path = tmp.path().join("test.zip");
        make_zip(
            &zip_path,
            &[
                ("myproject/", b""),
                ("myproject/Cargo.toml", b"[package]"),
                ("myproject/src/", b""),
                ("myproject/src/main.rs", b"fn main(){}"),
            ],
        );
        let project = tmp.path().join("project");
        extract_zip(&zip_path, &project).unwrap();

        assert!(project.join("Cargo.toml").exists());
        assert!(project.join("src").join("main.rs").exists());
        assert!(
            !project.join("myproject").exists(),
            "トップディレクトリが残ってはいけない"
        );
    }

    #[test]
    fn no_strip_when_flat_structure() {
        let tmp = tempdir().unwrap();
        let zip_path = tmp.path().join("flat.zip");
        make_zip(
            &zip_path,
            &[
                ("Cargo.toml", b"[package]"),
                ("src/main.rs", b"fn main(){}"),
            ],
        );
        let project = tmp.path().join("project");
        extract_zip(&zip_path, &project).unwrap();

        assert!(project.join("Cargo.toml").exists());
        assert!(project.join("src").join("main.rs").exists());
    }

    #[test]
    fn no_strip_when_multiple_top_dirs() {
        let tmp = tempdir().unwrap();
        let zip_path = tmp.path().join("multi.zip");
        make_zip(&zip_path, &[("a/file.txt", b"aaa"), ("b/file.txt", b"bbb")]);
        let project = tmp.path().join("project");
        extract_zip(&zip_path, &project).unwrap();

        assert!(project.join("a").join("file.txt").exists());
        assert!(project.join("b").join("file.txt").exists());
    }
}

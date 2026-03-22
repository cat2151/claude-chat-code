/// ZIP 展開に責任を持つ。
/// トップレベルに単一ディレクトリがある場合はそれを剥いて展開する。

use anyhow::Result;
use std::{collections::HashSet, fs, io, path::Path};

/// zip_path の zip を project_dir に上書き展開する。
/// トップレベルにディレクトリが 1 つだけある場合はそのディレクトリを剥いて展開する。
pub fn extract_zip(zip_path: &Path, project_dir: &Path) -> Result<()> {
    fs::create_dir_all(project_dir)?;

    let strip_prefix = {
        let file = fs::File::open(zip_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        detect_single_top_dir(&mut archive)
    };

    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut zf = archive.by_index(i)?;
        let mangled = zf.mangled_name();

        let rel = if let Some(ref prefix) = strip_prefix {
            match mangled.strip_prefix(prefix) {
                Ok(r) if r.components().next().is_some() => r.to_path_buf(),
                _ => continue,
            }
        } else {
            mangled.to_path_buf()
        };

        let out_path = project_dir.join(&rel);

        if zf.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut zf, &mut outfile)?;
        }
    }
    Ok(())
}

/// zip 内のトップレベルエントリを調べ、
/// ディレクトリが 1 つだけ（かつファイルがトップレベルに存在しない）場合に
/// そのディレクトリ名を返す。それ以外は None。
fn detect_single_top_dir(archive: &mut zip::ZipArchive<fs::File>) -> Option<std::path::PathBuf> {
    let mut top_dirs: HashSet<String> = HashSet::new();
    let mut has_top_file = false;

    for i in 0..archive.len() {
        let zf = archive.by_index(i).ok()?;
        let mangled = zf.mangled_name();
        let top = mangled.components().next()?;
        let top_name = top.as_os_str().to_string_lossy().to_string();

        if zf.is_dir() {
            if mangled.components().count() == 1 {
                top_dirs.insert(top_name);
            }
        } else if mangled.components().count() == 1 {
            has_top_file = true;
        } else {
            top_dirs.insert(top_name);
        }
    }

    if !has_top_file && top_dirs.len() == 1 {
        Some(std::path::PathBuf::from(top_dirs.into_iter().next().unwrap()))
    } else {
        None
    }
}

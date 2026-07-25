use std::path::{Path, PathBuf};

/// Path resolver that generates unique output filenames.
pub struct PathResolver;

impl PathResolver {
    /// Generates a unique target path of the form:
    /// `<basename>-decoded[(incremental index if dup)].<ext>`
    pub fn generate_unique_path(original: &Path) -> PathBuf {
        let parent = original.parent().unwrap_or_else(|| Path::new("."));
        let stem = original.file_stem().and_then(|s| s.to_str()).unwrap_or("decoded");
        let ext = original.extension().and_then(|s| s.to_str()).unwrap_or("");

        let mut idx = 0;
        loop {
            let filename = if idx == 0 {
                if ext.is_empty() { format!("{}-decoded", stem) } else { format!("{}-decoded.{}", stem, ext) }
            } else {
                if ext.is_empty() { format!("{}-decoded({})", stem, idx) } else { format!("{}-decoded({}).{}", stem, idx, ext) }
            };

            let candidate = parent.join(&filename);
            if !candidate.exists() {
                return candidate;
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_path_resolver_basic() {
        let path = Path::new("dummy_file.txt");
        let resolved = PathResolver::generate_unique_path(path);
        assert_eq!(resolved, Path::new("dummy_file-decoded.txt"));
    }

    #[test]
    fn test_path_resolver_duplicate() {
        let temp_dir = Path::new("test_resolver_dup_dir");
        let _ = fs::create_dir_all(temp_dir);

        let input_path = temp_dir.join("sample.txt");
        let first_expected = temp_dir.join("sample-decoded.txt");
        let second_expected = temp_dir.join("sample-decoded(1).txt");

        let res1 = PathResolver::generate_unique_path(&input_path);
        assert_eq!(res1, first_expected);

        fs::write(&first_expected, b"").unwrap();

        let res2 = PathResolver::generate_unique_path(&input_path);
        assert_eq!(res2, second_expected);

        let _ = fs::remove_dir_all(temp_dir);
    }
}

use std::path::{Path, PathBuf};

/// Path resolver that generates unique output filenames.
pub struct PathResolver;

impl PathResolver {
    /// Generates a unique target path of the form:
    /// `<basename>-decoded[(incremental index if dup)].<ext>`
    pub fn generate_unique_path(original_path: &Path) -> PathBuf {
        let parent = original_path.parent().unwrap_or_else(|| Path::new("."));
        let file_stem = original_path.file_stem().and_then(|s| s.to_str()).unwrap_or("decoded");
        let extension = original_path.extension().and_then(|s| s.to_str()).unwrap_or("");

        let mut index = 0;
        loop {
            let suffix = if index == 0 {
                "-decoded".to_string()
            } else {
                format!("-decoded({})", index)
            };

            let mut new_filename = format!("{}{}", file_stem, suffix);
            if !extension.is_empty() {
                new_filename.push('.');
                new_filename.push_str(extension);
            }

            let candidate = parent.join(&new_filename);
            if !candidate.exists() {
                return candidate;
            }
            index += 1;
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
        // Should default to dummy_file-decoded.txt (assuming it doesn't exist)
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

        // Generate when no file exists
        let res1 = PathResolver::generate_unique_path(&input_path);
        assert_eq!(res1, first_expected);

        // Create the first decoded file
        fs::write(&first_expected, b"").unwrap();

        // Generate again, should increment to (1)
        let res2 = PathResolver::generate_unique_path(&input_path);
        assert_eq!(res2, second_expected);

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }
}

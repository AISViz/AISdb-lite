use std::fs::read_dir;

/// yields sorted vector of files in dirname with a matching file extension.
pub fn glob_dir(dirname: std::path::PathBuf, matching: &str) -> Option<Vec<String>> {
    println!("{:?}", dirname);
    let mut fnames = read_dir(dirname)
        .expect("glob dir")
        .map(|f| f.unwrap().path().display().to_string())
        .filter(|f| &f[f.len() - matching.chars().count()..] == matching)
        .collect::<Vec<String>>()
        .to_vec();
    fnames.sort();
    Some(fnames)
}

#[cfg(test)]
mod tests {
    use super::glob_dir;

    #[test]
    fn test_glob_dir() {
        let _ = glob_dir(std::path::PathBuf::from("src/"), "rs");
    }
}

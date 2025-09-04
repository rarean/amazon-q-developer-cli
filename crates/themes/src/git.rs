use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::time::{
    Duration,
    Instant,
};

const GIT_TIMEOUT_MS: u64 = 500; // 500ms timeout for git operations
const CACHE_DURATION_MS: u64 = 2000; // 2 second cache duration

lazy_static::lazy_static! {
    static ref GIT_CACHE: Mutex<HashMap<String, (GitInfo, Instant)>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub clean: bool,
    pub staged: bool,
    pub modified: bool,
    pub untracked: bool,
    pub ahead: u32,
    pub behind: u32,
}

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub is_repo: bool,
    pub branch: Option<String>,
    pub status: Option<GitStatus>,
}

impl GitInfo {
    pub fn detect(path: &Path) -> Self {
        // Handle invalid paths gracefully
        if !path.exists() {
            return Self::empty();
        }

        let path_str = path.to_string_lossy().to_string();

        // Check cache first
        if let Ok(cache) = GIT_CACHE.lock() {
            if let Some((cached_info, timestamp)) = cache.get(&path_str) {
                if timestamp.elapsed().as_millis() < CACHE_DURATION_MS as u128 {
                    return cached_info.clone();
                }
            }
        }

        // Compute fresh git info
        let git_info = Self::detect_fresh(path);

        // Update cache
        if let Ok(mut cache) = GIT_CACHE.lock() {
            cache.insert(path_str, (git_info.clone(), Instant::now()));
        }

        git_info
    }

    pub fn empty() -> Self {
        Self {
            is_repo: false,
            branch: None,
            status: None,
        }
    }

    fn detect_fresh(path: &Path) -> Self {
        let is_repo = Self::is_git_repo(path);
        let branch = if is_repo {
            Self::get_branch_with_fallback(path)
        } else {
            None
        };
        let status = if is_repo { Self::get_status(path) } else { None };

        Self {
            is_repo,
            branch,
            status,
        }
    }

    fn run_git_command(args: &[&str], path: &Path) -> Option<String> {
        use std::process::Stdio;
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let path = path.to_path_buf();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();

        thread::spawn(move || {
            let output = Command::new("git")
                .args(&args)
                .current_dir(&path)
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .output();

            let _ = tx.send(output);
        });

        match rx.recv_timeout(Duration::from_millis(GIT_TIMEOUT_MS)) {
            Ok(Ok(output)) if output.status.success() => String::from_utf8(output.stdout)
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
            _ => None,
        }
    }

    fn is_git_repo(path: &Path) -> bool {
        path.join(".git").exists() || Self::run_git_command(&["rev-parse", "--git-dir"], path).is_some()
    }

    fn get_status(path: &Path) -> Option<GitStatus> {
        let status_output = Self::run_git_command(&["status", "--porcelain"], path)?;
        let (ahead, behind) = Self::get_ahead_behind(path);
        Some(Self::parse_status(&status_output, ahead, behind))
    }

    fn get_ahead_behind(path: &Path) -> (u32, u32) {
        Self::run_git_command(&["rev-list", "--left-right", "--count", "HEAD...@{upstream}"], path)
            .and_then(|output| {
                let parts: Vec<&str> = output.split_whitespace().collect();
                if parts.len() == 2 {
                    let ahead = parts[0].parse().unwrap_or(0);
                    let behind = parts[1].parse().unwrap_or(0);
                    Some((ahead, behind))
                } else {
                    None
                }
            })
            .unwrap_or((0, 0))
    }

    fn parse_status(status_output: &str, ahead: u32, behind: u32) -> GitStatus {
        let mut staged = false;
        let mut modified = false;
        let mut untracked = false;

        for line in status_output.lines() {
            if line.len() >= 2 {
                let index_status = line.chars().nth(0).unwrap_or(' ');
                let worktree_status = line.chars().nth(1).unwrap_or(' ');

                // Check staged changes
                if matches!(index_status, 'A' | 'M' | 'D' | 'R' | 'C') {
                    staged = true;
                }

                // Check modified files
                if matches!(worktree_status, 'M' | 'D') {
                    modified = true;
                }

                // Check untracked files
                if matches!(index_status, '?') {
                    untracked = true;
                }
            }
        }

        let clean = !staged && !modified && !untracked;

        GitStatus {
            clean,
            staged,
            modified,
            untracked,
            ahead,
            behind,
        }
    }

    fn get_branch_with_fallback(path: &Path) -> Option<String> {
        // Try git branch --show-current first
        if let Some(branch) = Self::run_git_command(&["branch", "--show-current"], path) {
            return Some(branch);
        }

        // Fallback to symbolic-ref
        if let Some(branch) = Self::run_git_command(&["symbolic-ref", "--short", "HEAD"], path) {
            return Some(branch);
        }

        // Fallback to describe for detached HEAD
        if let Some(desc) = Self::run_git_command(&["describe", "--tags", "--always"], path) {
            return Some(format!("HEAD~{}", desc));
        }

        // Final fallback to short SHA
        Self::run_git_command(&["rev-parse", "--short", "HEAD"], path).map(|sha| format!("#{}", sha))
    }

    #[allow(dead_code)]
    fn get_current_branch(path: &Path) -> Option<String> {
        Self::run_git_command(&["branch", "--show-current"], path)
    }

    #[allow(dead_code)]
    fn get_symbolic_ref(path: &Path) -> Option<String> {
        Self::run_git_command(&["symbolic-ref", "--short", "HEAD"], path)
    }

    #[allow(dead_code)]
    fn get_describe(path: &Path) -> Option<String> {
        Self::run_git_command(&["describe", "--tags", "--always"], path)
    }

    #[allow(dead_code)]
    fn get_short_sha(path: &Path) -> Option<String> {
        Self::run_git_command(&["rev-parse", "--short", "HEAD"], path).map(|sha| format!("#{}", sha))
    }
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_non_git_directory() {
        let temp_dir = std::env::temp_dir().join("non_git_test");
        let _ = fs::create_dir_all(&temp_dir);

        let git_info = GitInfo::detect(&temp_dir);
        assert!(!git_info.is_repo);
        assert!(git_info.branch.is_none());
        assert!(git_info.status.is_none());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_invalid_path() {
        let invalid_path = Path::new("/nonexistent/path");
        let git_info = GitInfo::detect(invalid_path);
        assert!(!git_info.is_repo);
        assert!(git_info.branch.is_none());
        assert!(git_info.status.is_none());
    }

    #[test]
    fn test_empty_git_info() {
        let empty = GitInfo::empty();
        assert!(!empty.is_repo);
        assert!(empty.branch.is_none());
        assert!(empty.status.is_none());
    }

    #[test]
    fn test_git_status_parsing() {
        let status_output = " M modified.txt\nA  added.txt\n?? untracked.txt\n";
        let status = GitInfo::parse_status(status_output, 1, 2);

        assert!(!status.clean);
        assert!(status.staged);
        assert!(status.modified);
        assert!(status.untracked);
        assert_eq!(status.ahead, 1);
        assert_eq!(status.behind, 2);
    }

    #[test]
    fn test_clean_git_status() {
        let status_output = "";
        let status = GitInfo::parse_status(status_output, 0, 0);

        assert!(status.clean);
        assert!(!status.staged);
        assert!(!status.modified);
        assert!(!status.untracked);
        assert_eq!(status.ahead, 0);
        assert_eq!(status.behind, 0);
    }
}

//! Native asset discovery is identical for debug and release builds.
use bevy::prelude::*;
use std::path::{Path, PathBuf};

pub fn plugin() -> AssetPlugin {
    let cwd =
        std::env::current_dir().expect("Cannot determine working directory for asset discovery");
    let executable = std::env::current_exe().ok();
    let override_root = std::env::var_os("BEVY_ASSET_ROOT").map(PathBuf::from);
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = resolve(
        override_root.as_deref(),
        executable.as_deref(),
        &cwd,
        &workspace,
    );
    info!("ASSETS {}", path.display());
    AssetPlugin {
        file_path: path.to_string_lossy().into_owned(),
        ..default()
    }
}

fn resolve(
    override_root: Option<&Path>,
    executable: Option<&Path>,
    cwd: &Path,
    workspace: &Path,
) -> PathBuf {
    // An explicit override must never silently fall back to another asset set.
    if let Some(root) = override_root {
        return cwd.join(root).join("assets");
    }
    if let Some(parent) = executable.and_then(Path::parent) {
        let packaged = parent.join("assets");
        if packaged.is_dir() {
            return packaged;
        }
    }
    let local = cwd.join("assets");
    if local.is_dir() {
        return local;
    }
    // Keep this path even when missing, so Bevy reports the attempted location.
    workspace.join("assets")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "saltline-assets-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
        fn directory(&self, name: &str) -> PathBuf {
            let path = self.0.join(name);
            std::fs::create_dir_all(&path).unwrap();
            path
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn packaged_assets_win_over_working_directory_and_workspace() {
        let f = Fixture::new();
        let packaged = f.directory("package/assets");
        f.directory("cwd/assets");
        f.directory("workspace/assets");
        assert_eq!(
            resolve(
                None,
                Some(&f.0.join("package/game")),
                &f.0.join("cwd"),
                &f.0.join("workspace")
            ),
            packaged
        );
    }

    #[test]
    fn repository_launch_uses_cwd_assets_without_a_packaged_directory() {
        let f = Fixture::new();
        let assets = f.directory("repository/assets");
        assert_eq!(
            resolve(
                None,
                Some(&f.0.join("repository/target/release/game")),
                &f.0.join("repository"),
                &f.0.join("other-workspace")
            ),
            assets
        );
    }

    #[test]
    fn launch_from_elsewhere_falls_back_to_build_workspace() {
        let f = Fixture::new();
        let assets = f.directory("workspace/assets");
        assert_eq!(
            resolve(
                None,
                Some(&f.0.join("workspace/target/debug/game")),
                &f.directory("elsewhere"),
                &f.0.join("workspace")
            ),
            assets
        );
    }

    #[test]
    fn explicit_override_is_respected_even_when_missing() {
        let f = Fixture::new();
        f.directory("package/assets");
        f.directory("cwd/assets");
        let missing = f.0.join("missing");
        assert_eq!(
            resolve(
                Some(&missing),
                Some(&f.0.join("package/game")),
                &f.0.join("cwd"),
                &f.0
            ),
            missing.join("assets")
        );
        assert_eq!(
            resolve(Some(Path::new("relative-root")), None, &f.0, &f.0),
            f.0.join("relative-root/assets")
        );
    }

    #[test]
    fn missing_candidates_keep_workspace_path_for_error_reporting() {
        let f = Fixture::new();
        assert_eq!(
            resolve(None, None, &f.0, &f.0.join("missing-workspace")),
            f.0.join("missing-workspace/assets")
        );
    }
}

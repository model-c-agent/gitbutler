use anyhow::Result;

/// Open the GitButler GUI application for `possibly_project_dir`,
/// which must be a directory or trigger an error.
///
/// This expects that the GUI application is present and has correctly registered URL
/// schemes for the different channels.
pub fn open(possibly_project_dir: &std::path::Path) -> Result<()> {
    #[cfg(not(target_os = "wasi"))]
    {
        use anyhow::Context as _;
        use but_path::AppChannel;

        if !possibly_project_dir.is_dir() {
            anyhow::bail!(
                "Can only open the GUI on directories: '{not_dir}'",
                not_dir = possibly_project_dir.display()
            );
        }

        let channel = AppChannel::new();
        let absolute_path = std::fs::canonicalize(possibly_project_dir).with_context(|| {
            format!(
                "Failed to canonicalize path before opening the GUI: {}",
                possibly_project_dir.display()
            )
        })?;
        channel.open(&absolute_path)?;
        Ok(())
    }
    #[cfg(target_os = "wasi")]
    {
        let _ = possibly_project_dir;
        anyhow::bail!("Opening the GUI is not supported on this platform")
    }
}

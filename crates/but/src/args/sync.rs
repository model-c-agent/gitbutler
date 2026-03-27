/// Arguments for sync management commands.
#[derive(Debug, clap::Parser)]
pub struct Platform {
    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Pause background sync temporarily.
    ///
    /// Creates a marker file that tells GitButler to skip background sync
    /// operations (fetch, PR refresh, CI checks). The pause auto-expires
    /// after the specified duration (default: 1 hour).
    Pause {
        /// How long to pause sync. Accepts formats like "30m", "2h", "1h30m", "90m".
        /// Default: 1h
        #[clap(long, short = 'd', default_value = "1h")]
        duration: String,
    },
    /// Resume background sync.
    ///
    /// Removes the sync pause marker, allowing background sync to proceed
    /// on the next command that triggers it.
    Resume,
    /// Show whether background sync is currently paused.
    ///
    /// Displays the current sync state and, if paused, how much time
    /// remains before the pause auto-expires.
    Status,
}

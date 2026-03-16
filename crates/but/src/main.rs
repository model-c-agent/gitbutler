#[cfg(not(target_os = "wasi"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "legacy")]
    gitbutler_repo_actions::askpass::disable();
    but::handle_args(std::env::args_os()).await
}

#[cfg(target_os = "wasi")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    but::handle_args(std::env::args_os()).await
}

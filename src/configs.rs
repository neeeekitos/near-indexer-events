use clap::Parser;

/// NEAR Indexer for Explorer
/// Watches for stream of blocks from the chain
#[derive(Parser, Debug)]
#[clap(
    version,
    author,
    about,
    disable_help_subcommand(true),
    propagate_version(true),
    next_line_help(true)
)]
pub(crate) struct Opts {
    /// Enabled Indexer for Explorer debug level of logs
    #[clap(long)]
    pub debug: bool,
    /// AWS S3 bucket name to get the stream from
    #[clap(long)]
    pub s3_bucket_name: String,
    /// AWS S3 bucket region
    #[clap(long)]
    pub s3_region_name: String,
    /// Block height to start the stream from
    #[clap(long, short)]
    pub start_block_height: u64,
    #[clap(long, short)]
    pub near_archival_rpc_url: String,
}

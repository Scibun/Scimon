use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Flags {
    #[arg(short, long)]
    /// URL for make scraping at page
    pub url: Option<String>,

    #[arg(long)]
    /// Select the scrape mode
    pub scrape: bool,

    #[arg(short, long)]
    /// Execute a list of tasks or run a specific task
    pub run: Option<String>,

    #[arg(long)]
    /// No ignore any pdf files
    pub no_ignore: bool,

    #[arg(long)]
    /// Disable checksum generator
    pub no_checksum: bool,

    #[arg(long)]
    /// Disable checksum validate
    pub no_checksum_validate: bool,

    #[arg(long)]
    /// Disable the comments and !debug directive
    pub no_comments: bool,

    #[arg(long)]
    /// Disable !open_link directive
    pub no_open_link: bool,

    #[arg(long)]
    /// Disable !readme directive
    pub no_readme: bool,

    #[arg(long)]
    /// Your settings
    pub options: Option<String>,
}

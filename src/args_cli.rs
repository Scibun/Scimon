use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Flags {
    #[arg(long)]
    /// The paimon file to create a new Monlib list
    pub file: Option<String>,

    #[arg(long)]
    /// URL for make scraping at page
    pub url: Option<String>,

    #[arg(long)]
    /// Select the scrape mode
    pub scrape: bool,

    #[arg(long)]
    /// Title of a new Monlib list
    pub title: Option<String>,

    #[arg(long)]
    /// Privacy of a new Monlib list
    pub privacy: Option<String>,

    #[arg(short, long)]
    /// Run a Monlib list or execute a specific list
    pub run: Option<String>,

    #[arg(long)]
    /// No ignore any pdf files
    pub noignore: bool,

    #[arg(long)]
    /// Disable the comments and !debug macro
    pub no_comments: bool,

    #[arg(long)]
    /// Your Kindle email for send the ebooks for your account
    pub kindle: Option<String>,

    #[arg(long)]
    /// Your Paimon settings
    pub options: Option<String>,

    #[arg(long)]
    /// Inspect the pdf files at library selected
    pub inspect: bool,

    #[arg(short, long)]
    /// Publish a new library in your Monlib account
    pub publish: bool,
}

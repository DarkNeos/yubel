use argh::FromArgs;

/// Params for yubel
#[derive(Debug, FromArgs)]
struct Params {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Command {
    Protoc(ProtocCmd),
}

/// Command to generate protobuf code
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "protoc")]
struct ProtocCmd {
    /// repo name to generate protobuf code
    #[argh(option)]
    pub repo: Option<String>,
    #[argh(subcommand)]
    pub raw: Option<RawProtoCmd>,
}

/// Specify protoc command
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "raw")]
struct RawProtoCmd {
    /// source directory
    #[argh(option)]
    pub src: String,
    /// out directory
    #[argh(option)]
    pub out: String,
    /// specify language
    #[argh(option)]
    pub lang: String,
}

fn main() {
    let params: Params = argh::from_env();

    println!("{:?}", params);
}

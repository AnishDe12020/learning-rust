use color_eyre::eyre::Result;
use structopt::StructOpt;

/// A digital garden rust CLI
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    // write something to the garden
    ///
    /// This command will open your $EDITOR, wait for you
    /// to finish writing, and then write the contents to the
    /// garden.
    Write {
        /// Optionally set a title
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(opt);
    todo!();
}

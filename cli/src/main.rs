use structopt::StructOpt;
mod plot;

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA_SHORT"),
    " ",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

#[derive(StructOpt)]
#[structopt(
    about = "Visualize ping to a server over time",
    version = VERSION
)]
pub struct Args {}

#[tokio::main]
async fn main() {
    let _args = Args::from_args();
    plot::Plot::run().await.unwrap();
}

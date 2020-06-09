use structopt::StructOpt;
mod show;
use show::Show;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let show = Show::new(args.show);
    let eps =
        if args.all {
            show.eps_all().await
        } else {
            show.eps(args.page, args.limit).await?
        };

    for ep in eps {
        println!("{}", ep);
        println!("\n");
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "eztv cli")]
struct Opt {
    #[structopt(short, long, help = "Show's id at IMDB")]
    show: u32,
    #[structopt(short, long, default_value = "1", help = "Page")]
    page: u32,
    #[structopt(short, long, default_value = "15", help = "Amount of torrents per page")]
    limit: u32,
    #[structopt(short, long, help = "Grabs all shows")]
    all: bool
}

use structopt::StructOpt;
mod show;
use show::Show;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let show = Show::new(args.show);
    let eps =
        if let Some(season) = args.season {
            if let Some(episode) = args.episode {
                show.ep(&season.to_string(), &episode.to_string()).await
            } else {
                show.eps_by_season(&season.to_string()).await
            }
        } else if args.all {
            show.eps_all().await
        } else {
            show.eps(args.page, args.limit).await?
        };

    if eps.is_empty() {
        println!("\
 [0;31;5m _______ _______      _______ _______ _______ ______ _______ __
|    |  |       |    |   |   |   _   |_     _|      |   |   |  |
|       |   -   |    |       |       | |   | |   ---|       |__|
|__|____|_______|    |__|_|__|___|___| |___| |______|___|___|__|

[0;31mShit might be unaccessible through API. This happens with (~5+ years) old \
torrents that only have magnet but no '.torrent' link.[0;0m");
        return Ok(());
    }

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
    #[structopt(long, help = "Which season?")]
    season: Option<u32>,
    #[structopt(long, help = "Which episode?")]
    episode: Option<u32>,
    #[structopt(short, long, default_value = "1", help = "Page")]
    page: u32,
    #[structopt(short, long, default_value = "15", help = "Amount of torrents per page")]
    limit: u32,
    #[structopt(short, long, help = "Grabs all shows")]
    all: bool
}

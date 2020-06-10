use std::fmt;
use serde::{ Serialize, Deserialize };
use reqwest::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    filename: String,
    magnet_url: String,
    title: String,
    season: String,
    episode: String,
    size_bytes: String
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[0;93m->[0;94m {} :: {}MiB\n\n[0;1m{}",
               self.title,
               self.size_bytes.parse::<u32>().unwrap() / 1048576,
               self.magnet_url
              )
    }
}

pub struct Show {
    id: u32
}

impl Show {
    pub fn new(id: u32) -> Show {
        let show = Show { id };
        show
    }

    pub async fn eps(&self, page: u32, limit: u32) -> Result<Vec<Episode>, Box<dyn std::error::Error>> {
        let mut episodes: Vec<Episode> = vec!();

        let show: serde_json::Value = serde_json::from_str(&reqwest::get(
            Url::parse(
                &format!("https://eztv.io/api/get-torrents?imdb_id={}&page={}&limit={}",
                         self.id, page, limit)
            ).unwrap()
        ).await?.text().await?)?;
        episodes.append(&mut serde_json::from_value::<Vec<Episode>>(show["torrents"].clone())?);

        Ok(episodes)
    }

    pub async fn eps_all(&self) -> Vec<Episode> {
        let mut all_eps: Vec<Episode> = vec!();
        let mut page = 1;
        while let Ok(ep) = &mut self.eps(page, 100).await {
            all_eps.append(ep);
            page += 1;
        }
        all_eps
    }

    pub async fn eps_by_season(&self, season: &str) -> Vec<Episode> {
        let all_eps = self.eps_all().await;
        all_eps.into_iter().filter(|ep| ep.season == season).collect()
    }

    pub async fn ep(&self, season: &str, episode: &str) -> Vec<Episode> {
        let eps_by_season = self.eps_by_season(season).await;
        eps_by_season.into_iter().filter(|ep| ep.episode == episode).collect()
    }
}

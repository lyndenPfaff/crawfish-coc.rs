/* --- IMPORTS --- */

use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;

use crate::model::*;
use crate::Error;

/* --- ------- --- */



/* --- STRUCTS --- */

pub struct Client {
    token: String,
    client: reqwest::Client,
}

/* --- ------- --- */



/* --- IMPLS --- */

impl Client {
    pub fn new(token: impl Into<String>) -> Client {
        Client { token: token.into(), client: reqwest::Client::new() }
    }

    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let response = self.client.get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .send().await.map_err(Error::HTTPError)?;

        match response.status() {
            StatusCode::OK => {
                serde_json::from_str(
                    &response.text().await.map_err(Error::HTTPError)?
                ).map_err(Error::ParseError)
            },
            _ => {
                let err = serde_json::from_str(
                    &response.text().await.map_err(Error::HTTPError)?
                ).map_err(Error::ParseError)?;
                Err(Error::ClientError(err))
            }
        }
    }

    pub async fn get_cwl_group(&self, tag: &str) -> Result<ClanWarLeagueGroup, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/currentwar/leaguegroup/")).await
    }

    pub async fn get_cwl_war(&self, wartag: &str) -> Result<ClanWarLeagueGroup, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clanwarleagues/wars/{wartag}/")).await
    }

    pub async fn get_current_war(&self, tag: &str) -> Result<ClanWar, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/currentwar/")).await
    }

    pub async fn get_clan(&self, tag: &str) -> Result<Clan, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/")).await
    }

    pub async fn get_clan_members(&self, tag: &str) -> Result<Vec<ClanMember>, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/members/")).await
    }

    pub async fn get_capital_raid_seasons(&self, tag: &str) -> Result<Vec<ClanCapitalRaidSeason>, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/capitalraidseasons/")).await
    }



    pub async fn get_player(&self, tag: &str) -> Result<Player, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/players/{tag}/")).await
    }

    pub async fn get_player_battle_log(&self, tag: &str) -> Result<Vec<BattleLogEntry>, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/players/{tag}/battlelog/")).await
    }

    pub async fn get_player_league_history(&self, tag: &str) -> Result<Vec<LeagueSeasonResult>, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/players/{tag}/leaguehistory/")).await
    }



    pub async fn get_league_tier(&self, id: i64) -> Result<LeagueTier, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/leaguetiers/{id}/")).await
    }

    pub async fn league_tiers(&self) -> Result<Vec<CapitalLeague>, Error> {
        self.get("https://api.clashofclans.com/v1/leaguetiers/").await
    }



    pub async fn capital_leagues(&self) -> Result<Vec<CapitalLeague>, Error> {
        self.get("https://api.clashofclans.com/v1/capitalleagues/").await
    }

    pub async fn get_capital_league(&self, id: i64) -> Result<CapitalLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/capitalleagues/{id}/")).await
    }



    pub async fn get_builder_base_league(&self, id: i64) -> Result<BuilderBaseLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/builderbaseleagues/{id}/")).await
    }

    pub async fn builder_base_leagues(&self) -> Result<Vec<BuilderBaseLeague>, Error> {
        // /builderbaseleagues
        self.get("https://api.clashofclans.com/v1/builderbaseleagues/").await
    }



    pub async fn war_leagues(&self) -> Result<Vec<WarLeague>, Error> {
        // /warleagues
        self.get("https://api.clashofclans.com/v1/warleagues/").await
    }

    pub async fn get_war_league(&self, id: i64) -> Result<Vec<WarLeague>, Error> {
        // /warleagues/{id}
        self.get(&format!("https://api.clashofclans.com/v1/warleagues/{id}")).await
    }
}

/* --- ----- --- */

/* --- MODULES --- */

mod builder;

/* --- ------- --- */



/* --- IMPORTS --- */

use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::client::builder::*;
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

    pub (crate) async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
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



    pub async fn get_player(&self, tag: &str) -> Result<Player, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/players/{tag}/")).await
    }

    pub async fn get_player_battle_log(&self, tag: &str) -> Result<Vec<BattleLogEntry>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<BattleLogEntry> }
        Ok(self.get::<W>(&format!("https://api.clashofclans.com/v1/players/{tag}/battlelog/")).await?.items)
    }

    pub async fn get_player_league_history(&self, tag: &str) -> Result<Vec<LeagueSeasonResult>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<LeagueSeasonResult> }
        Ok(self.get::<W>(&format!("https://api.clashofclans.com/v1/players/{tag}/leaguehistory/")).await?.items)
    }



    pub async fn get_league_tier(&self, id: i64) -> Result<LeagueTier, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/leaguetiers/{id}/")).await
    }

    pub async fn league_tiers(&self) -> Result<Vec<LeagueTier>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<LeagueTier> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/leaguetiers/").await?.items)
    }



    pub async fn capital_leagues(&self) -> Result<Vec<CapitalLeague>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<CapitalLeague> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/capitalleagues/").await?.items)
    }

    pub async fn get_capital_league(&self, id: i64) -> Result<CapitalLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/capitalleagues/{id}/")).await
    }



    pub async fn get_builder_base_league(&self, id: i64) -> Result<BuilderBaseLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/builderbaseleagues/{id}/")).await
    }

    pub async fn builder_base_leagues(&self) -> Result<Vec<BuilderBaseLeague>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<BuilderBaseLeague> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/builderbaseleagues/").await?.items)
    }



    pub async fn war_leagues(&self) -> Result<Vec<WarLeague>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<WarLeague> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/warleagues/").await?.items)
    }

    pub async fn get_war_league(&self, id: i64) -> Result<WarLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/warleagues/{id}/")).await
    }

    pub async fn get_location(&self, id: i64) -> Result<Location, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/locations/{id}/")).await
    }

    pub async fn get_current_gold_pass_season(&self) -> Result<GoldPassSeason, Error> {
        self.get("https://api.clashofclans.com/v1/goldpass/seasons/current/").await
    }



    pub fn search_clan_war_log<'a>(&'a self, clan_tag: impl Into<String>) -> WarLogSearch<'a> {
        WarLogSearch::new(self, clan_tag.into())
    }

    pub fn search_clans<'a>(&'a self) -> ClanSearch<'a> {
        ClanSearch::new(self)
    }

    pub fn search_clan_members<'a>(&'a self, clan_tag: impl Into<String>) -> ClanMemberSearch<'a> {
        ClanMemberSearch::new(self, clan_tag.into())
    }

    pub fn search_capital_raid_seasons<'a>(&'a self, clan_tag: impl Into<String>) -> CapitalRaidSeasonSearch<'a> {
        CapitalRaidSeasonSearch::new(self, clan_tag.into())
    }

    pub fn search_player_battlelog<'a>(&'a self, tag: impl Into<String>) -> PlayerBattleLogSearch<'a> {
        PlayerBattleLogSearch::new(self, tag.into())
    }

    pub fn search_location_clan_rankings<'a>(&'a self, location_id: i64) -> LocationalClanRankingsSearch<'a> {
        LocationalClanRankingsSearch::new(self, location_id)
    }

    pub fn search_location_player_rankings<'a>(&'a self, location_id: i64) -> LocationalPlayerRankingsSearch<'a> {
        LocationalPlayerRankingsSearch::new(self, location_id)
    }

    pub fn search_location_player_builder_rankings<'a>(&'a self, location_id: i64) -> LocationalPlayerBuilderRankingsSearch<'a> {
        LocationalPlayerBuilderRankingsSearch::new(self, location_id)
    }

    pub fn search_location_clan_builder_rankings<'a>(&'a self, location_id: i64) -> LocationalClanBuilderRankingsSearch<'a> {
        LocationalClanBuilderRankingsSearch::new(self, location_id)
    }

    pub fn search_locations<'a>(&'a self) -> LocationsSearch<'a> {
        LocationsSearch::new(self)
    }

    pub fn search_location_clan_capital_rankings<'a>(&'a self, location_id: i64) -> LocationalClanCapitalRankingsSearch<'a> {
        LocationalClanCapitalRankingsSearch::new(self, location_id)
    }

}

/* --- ----- --- */

/* --- MODULES --- */

mod builder;

/* --- ------- --- */



/* --- IMPORTS --- */

use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::client::builder::{
    WarLogSearch, ClanSearch, ClanMemberSearch, CapitalRaidSeasonSearch,
    PlayerBattleLogSearch, LocationalClanRankingsSearch, LocationalPlayerRankingsSearch,
    LocationalPlayerBuilderRankingsSearch, LocationalClanBuilderRankingsSearch,
    LocationsSearch, LocationalClanCapitalRankingsSearch
};
use crate::model::{
    ClanWarLeagueGroup, ClanWar, Clan, Player, BattleLogEntry, LeagueSeasonResult,
    LeagueTier, CapitalLeague, BuilderBaseLeague, WarLeague, Location, GoldPassSeason
};
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

        if response.status() == StatusCode::OK {
            serde_json::from_str(
                &response.text().await.map_err(Error::HTTPError)?
            ).map_err(Error::ParseError)
        } else {
            let err = serde_json::from_str(
                &response.text().await.map_err(Error::HTTPError)?
            ).map_err(Error::ParseError)?;
            Err(Error::ClientError(err))
        }
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_cwl_group(&self, tag: &str) -> Result<ClanWarLeagueGroup, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/currentwar/leaguegroup/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_cwl_war(&self, wartag: &str) -> Result<ClanWarLeagueGroup, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clanwarleagues/wars/{wartag}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_current_war(&self, tag: &str) -> Result<ClanWar, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/currentwar/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_clan(&self, tag: &str) -> Result<Clan, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/clans/{tag}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_player(&self, tag: &str) -> Result<Player, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/players/{tag}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_player_battle_log(&self, tag: &str) -> Result<Vec<BattleLogEntry>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<BattleLogEntry> }
        Ok(self.get::<W>(&format!("https://api.clashofclans.com/v1/players/{tag}/battlelog/")).await?.items)
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_player_league_history(&self, tag: &str) -> Result<Vec<LeagueSeasonResult>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<LeagueSeasonResult> }
        Ok(self.get::<W>(&format!("https://api.clashofclans.com/v1/players/{tag}/leaguehistory/")).await?.items)
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_league_tier(&self, id: i64) -> Result<LeagueTier, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/leaguetiers/{id}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn league_tiers(&self) -> Result<Vec<LeagueTier>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<LeagueTier> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/leaguetiers/").await?.items)
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn capital_leagues(&self) -> Result<Vec<CapitalLeague>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<CapitalLeague> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/capitalleagues/").await?.items)
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_capital_league(&self, id: i64) -> Result<CapitalLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/capitalleagues/{id}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_builder_base_league(&self, id: i64) -> Result<BuilderBaseLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/builderbaseleagues/{id}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn builder_base_leagues(&self) -> Result<Vec<BuilderBaseLeague>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<BuilderBaseLeague> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/builderbaseleagues/").await?.items)
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn war_leagues(&self) -> Result<Vec<WarLeague>, Error> {
        #[derive(Deserialize)]
        struct W { items: Vec<WarLeague> }
        Ok(self.get::<W>("https://api.clashofclans.com/v1/warleagues/").await?.items)
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_war_league(&self, id: i64) -> Result<WarLeague, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/warleagues/{id}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_location(&self, id: i64) -> Result<Location, Error> {
        self.get(&format!("https://api.clashofclans.com/v1/locations/{id}/")).await
    }

    /// # Errors
    /// Returns a [`Error::HTTPError`] if [`reqwest`] returned an error while sending the request to the API
    /// 
    /// Returns a [`Error::ParseError`] if [`serde`] failed to parse the received json object. Contact authors.
    /// 
    /// Returns a [`Error::ClientError`] if the API call returned successfully, but contained an error object
    pub async fn get_current_gold_pass_season(&self) -> Result<GoldPassSeason, Error> {
        self.get("https://api.clashofclans.com/v1/goldpass/seasons/current/").await
    }


    
    pub fn search_clan_war_log(&self, clan_tag: impl Into<String>) -> WarLogSearch<'_> {
        WarLogSearch::new(self, clan_tag.into())
    }

    #[must_use]
    pub fn search_clans(&self) -> ClanSearch<'_> {
        ClanSearch::new(self)
    }

    pub fn search_clan_members(&self, clan_tag: impl Into<String>) -> ClanMemberSearch<'_> {
        ClanMemberSearch::new(self, clan_tag.into())
    }

    pub fn search_capital_raid_seasons(&self, clan_tag: impl Into<String>) -> CapitalRaidSeasonSearch<'_> {
        CapitalRaidSeasonSearch::new(self, clan_tag.into())
    }

    pub fn search_player_battlelog(&self, tag: impl Into<String>) -> PlayerBattleLogSearch<'_> {
        PlayerBattleLogSearch::new(self, tag.into())
    }

    #[must_use]
    pub fn search_location_clan_rankings(&self, location_id: i64) -> LocationalClanRankingsSearch<'_> {
        LocationalClanRankingsSearch::new(self, location_id)
    }

    #[must_use]
    pub fn search_location_player_rankings(&self, location_id: i64) -> LocationalPlayerRankingsSearch<'_> {
        LocationalPlayerRankingsSearch::new(self, location_id)
    }

    #[must_use]
    pub fn search_location_player_builder_rankings(&self, location_id: i64) -> LocationalPlayerBuilderRankingsSearch<'_> {
        LocationalPlayerBuilderRankingsSearch::new(self, location_id)
    }

    #[must_use]
    pub fn search_location_clan_builder_rankings(&self, location_id: i64) -> LocationalClanBuilderRankingsSearch<'_> {
        LocationalClanBuilderRankingsSearch::new(self, location_id)
    }

    #[must_use]
    pub fn search_locations(&self) -> LocationsSearch<'_> {
        LocationsSearch::new(self)
    }

    #[must_use]
    pub fn search_location_clan_capital_rankings(&self, location_id: i64) -> LocationalClanCapitalRankingsSearch<'_> {
        LocationalClanCapitalRankingsSearch::new(self, location_id)
    }

}

/* --- ----- --- */

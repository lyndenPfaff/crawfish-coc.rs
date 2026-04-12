/* --- IMPORTS --- */

use crate::client::Client;
use crate::model::{ ClanWarLogEntry, PagingCursors, SearchResponse, Clan, ClanMember, ClanCapitalRaidSeason, BattleLogEntry, ClanRanking, PlayerRanking, PlayerBuilderBaseRanking, ClanBuilderBaseRanking, Location, ClanCapitalRanking };
use crate::Error;

/* --- ------- --- */



/* --- STRUCTS --- */

pub struct WarLogSearch<'a> {
    client: &'a Client,
    clan_tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct ClanSearch<'a> {
    client: &'a Client,
    name: Option<String>,
    /// Enum: [ `UNKNOWN`, `ALWAYS`, `MORE_THAN_ONCE_PER_WEEK`, `ONCE_PER_WEEK`, `LESS_THAN_ONCE_PER_WEEK`, `NEVER`, `ANY` ]
    war_frequency: Option<String>,
    location_id: Option<i64>,
    min_members: Option<i64>,
    max_members: Option<i64>,
    min_clan_points: Option<i64>,
    min_clan_level: Option<i64>,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
    label_ids: Vec<i64>,
}

pub struct ClanMemberSearch<'a> {
    client: &'a Client,
    clan_tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct CapitalRaidSeasonSearch<'a> {
    client: &'a Client,
    clan_tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct PlayerBattleLogSearch<'a> {
    client: &'a Client,
    tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalClanRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalPlayerRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalPlayerBuilderRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalClanBuilderRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationsSearch<'a> {
    client: &'a Client,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalClanCapitalRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

/* --- ------- --- */



/* --- IMPLS --- */

impl WarLogSearch<'_> {
    pub (crate) fn new(client: &Client, clan_tag: String) -> WarLogSearch<'_> {
        WarLogSearch { client, clan_tag, limit: None, after: None, before: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanWarLogEntry>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/clans/{}/warlog{}{}{}", self.clan_tag,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl ClanSearch<'_> {
    pub (crate) fn new(client: &Client) -> ClanSearch<'_> {
        ClanSearch {
            client, name: None, war_frequency: None,
            location_id: None, min_members: None, max_members: None,
            min_clan_points: None, min_clan_level: None, limit: None,
            after: None, before: None, label_ids: Vec::new()
        }
    }

    pub async fn send(self) -> Result<(Vec<Clan>, PagingCursors), Error> {

        let url = format!("https://api.clashofclans.com/v1/clans{}{}{}{}{}{}{}{}{}{}{}",
            if let Some(name) = self.name { format!("?name={name}") } else {String::new()},
            if let Some(war_frequency) = self.war_frequency { format!("?warFrequency={war_frequency}") } else {String::new()},
            if let Some(location_id) = self.location_id { format!("?locationId={location_id}") } else {String::new()},
            if let Some(min_members) = self.min_members { format!("?locationId={min_members}") } else {String::new()},
            if let Some(max_members) = self.max_members { format!("?locationId={max_members}") } else {String::new()},
            if let Some(min_clan_points) = self.min_clan_points { format!("?locationId={min_clan_points}") } else {String::new()},
            if let Some(min_clan_level) = self.min_clan_level { format!("?locationId={min_clan_level}") } else {String::new()},
            if let Some(limit) = self.limit { format!("?locationId={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?locationId={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?locationId={before}") } else {String::new()},
            if self.label_ids.is_empty() { String::new() }
            else { format!("?locationId={}",
                self.label_ids.into_iter()
                    .map(|l| format!("{l}"))
                    .collect::<Vec<_>>()
                    .join(","))
            },
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn war_frequency(mut self, war_frequency: String) -> Self {
        self.war_frequency= Some(war_frequency);
        self
    }

    pub fn location(mut self, location_id: i64) -> Self {
        self.location_id= Some(location_id);
        self
    }

    pub fn min_members(mut self, min: i64) -> Self {
        self.min_members = Some(min);
        self
    }

    pub fn max_members(mut self, max: i64) -> Self {
        self.max_members = Some(max);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }

    pub fn add_label_ids(mut self, ids: impl Into<Vec<i64>>) -> Self {
        self.label_ids.append(&mut ids.into());
        self
    }
}

impl ClanMemberSearch<'_> {
    pub (crate) fn new(client: &Client, clan_tag: String) -> ClanMemberSearch<'_> {
        ClanMemberSearch { client, clan_tag, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanMember>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/clans/{}/members{}{}{}", self.clan_tag,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl CapitalRaidSeasonSearch<'_> {
    pub (crate) fn new(client: &Client, clan_tag: String) -> CapitalRaidSeasonSearch<'_> {
        CapitalRaidSeasonSearch { client, clan_tag, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanCapitalRaidSeason>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/clans/{}/capitalraidseasons{}{}{}", self.clan_tag,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl PlayerBattleLogSearch<'_> {
    pub (crate) fn new(client: &Client, tag: String) -> PlayerBattleLogSearch<'_> {
        PlayerBattleLogSearch { client, tag, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<BattleLogEntry>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/clans/{}/battlelog{}{}{}", self.tag,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalClanRankingsSearch<'_> {
    pub (crate) fn new(client: &Client, location_id: i64) -> LocationalClanRankingsSearch<'_> {
        LocationalClanRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanRanking>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/clans{}{}{}", self.location_id,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalPlayerRankingsSearch<'_> {
    pub (crate) fn new(client: &Client, location_id: i64) -> LocationalPlayerRankingsSearch<'_> {
        LocationalPlayerRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<PlayerRanking>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/players{}{}{}", self.location_id,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );
        

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalPlayerBuilderRankingsSearch<'_> {
    pub (crate) fn new(client: &Client, location_id: i64) -> LocationalPlayerBuilderRankingsSearch<'_> {
        LocationalPlayerBuilderRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<PlayerBuilderBaseRanking>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/players-builder-base{}{}{}", self.location_id,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalClanBuilderRankingsSearch<'_> {
    pub (crate) fn new(client: &Client, location_id: i64) -> LocationalClanBuilderRankingsSearch<'_> {
        LocationalClanBuilderRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanBuilderBaseRanking>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/clans-builder-base{}{}{}", self.location_id,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationsSearch<'_> {
    pub (crate) fn new(client: &Client) -> LocationsSearch<'_> {
        LocationsSearch { client, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<Location>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/locations{}{}{}",
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalClanCapitalRankingsSearch<'_> {
    pub (crate) fn new(client: &Client, location_id: i64) -> LocationalClanCapitalRankingsSearch<'_> {
        LocationalClanCapitalRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanCapitalRanking>, PagingCursors), Error> {
        let url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/clans{}{}{}", self.location_id,
            if let Some(limit) = self.limit { format!("?limit={limit}") } else {String::new()},
            if let Some(after) = self.after { format!("?after={after}") } else {String::new()},
            if let Some(before) = self.before { format!("?before={before}") } else {String::new()},
        );

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

/* --- ----- --- */

use crate::domain::{Character, CharacterEntity, LightCone, LightConeEntity};
use async_trait::async_trait;
use eyre::Result;

pub mod hoyowiki_data_fetcher;
pub mod project_yatta_data_fetcher;

#[async_trait]
pub trait DataFetcher {
    async fn fetch_character_data(&self, character: &Character) -> Result<CharacterEntity>;
    async fn fetch_light_cone_data(&mut self, light_cone: &LightCone) -> Result<LightConeEntity>;
}

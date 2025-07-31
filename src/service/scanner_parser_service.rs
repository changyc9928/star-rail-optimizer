use crate::{
    data_fetcher::DataFetcher,
    domain::{
        Character, LightCone, LightConeEntity, RawCharacter, RawRelic, Relic, ScannerInput, Slot,
    },
};
use eyre::Result;
use futures::future::try_join_all;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct ScannerParserService {
    pub data_fetcher: Arc<Mutex<dyn DataFetcher + Send + Sync>>,
}

impl ScannerParserService {
    async fn populate_characters(
        &self,
        characters: &[RawCharacter],
    ) -> Result<HashMap<String, Character>> {
        let futures = characters
            .iter()
            .map(|character| {
                let data_fetcher = self.data_fetcher.clone();
                let character = character.clone();
                tokio::spawn(async move {
                    data_fetcher
                        .lock()
                        .await
                        .fetch_character_data(&character.clone())
                        .await
                })
            })
            .collect::<Vec<_>>();

        // Join all the futures and await their results
        let results = try_join_all(futures).await?;
        Ok(results
            .into_par_iter()
            .map(|r| {
                let character_entity = r?;
                Ok::<_, eyre::Report>((character_entity.id.clone(), character_entity))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .collect())
    }

    async fn populate_light_cone(
        &mut self,
        light_cones: &[LightCone],
    ) -> Result<HashMap<String, LightConeEntity>> {
        let futures = light_cones
            .iter()
            .map(|light_cone| {
                let data_fetcher = self.data_fetcher.clone();
                let light_cone = light_cone.clone();
                tokio::spawn(async move {
                    data_fetcher
                        .lock()
                        .await
                        .fetch_light_cone_data(&light_cone.clone())
                        .await
                })
            })
            .collect::<Vec<_>>();

        // Join all the futures and await their results
        let results = try_join_all(futures).await?;
        Ok(results
            .into_par_iter()
            .map(|r| {
                let light_cone_entity = r?;
                Ok::<_, eyre::Report>((
                    light_cone_entity._light_cone._uid.clone(),
                    light_cone_entity,
                ))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .collect())
    }

    fn categorise_relics(&self, relics: &[RawRelic]) -> Result<HashMap<Slot, Vec<Relic>>> {
        let mut relic_pool = HashMap::new();
        for relic in relics {
            relic_pool
                .entry(relic.slot.clone())
                .or_insert_with(Vec::new)
                .push(relic.clone().try_into()?);
        }
        Ok(relic_pool)
    }

    pub async fn parse_scanner_input(
        &mut self,
        scanner_input: &ScannerInput,
    ) -> Result<(
        HashMap<String, Character>,
        HashMap<String, LightConeEntity>,
        HashMap<Slot, Vec<Relic>>,
    )> {
        let characters = &scanner_input.characters;
        let light_cones = &scanner_input.light_cones;
        let relics = &scanner_input.relics;
        let character_entities = self.populate_characters(characters).await?;
        let light_cone_entities = self.populate_light_cone(light_cones).await?;
        let relics = self.categorise_relics(relics)?;
        Ok((character_entities, light_cone_entities, relics))
    }
}

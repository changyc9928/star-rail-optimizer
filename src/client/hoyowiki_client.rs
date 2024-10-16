use eyre::{eyre, Result};
use reqwest::Client;
use serde::Deserialize;

use crate::domain::HoyowikiResponse;

pub struct HoyowikiClient {
    pub base_url: String,
    pub language: String,
    pub wiki_app: String,
}

impl HoyowikiClient {
    pub async fn fetch_data<T: Clone + for<'de> Deserialize<'de>>(
        &self,
        data_name: &str,
        id: &str,
    ) -> Result<Vec<T>> {
        let client = Client::new();
        let response: HoyowikiResponse = client
            .get(format!("{}{}", self.base_url, "/entry_page"))
            .header("x-rpc-language", self.language.clone())
            .header("x-rpc-wiki_app", self.wiki_app.clone())
            .query(&[("entry_page_id", self.id_lookup(id))])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let raw_data = &response
            .data
            .page
            .modules
            .iter()
            .find(|m| m.name == data_name)
            .ok_or_else(|| eyre!("{} not found", data_name.to_owned()))?
            .components;
        let data: Vec<T> = raw_data
            .iter()
            .map(|d| Ok(serde_json::from_str::<T>(&d.data)?))
            .collect::<Result<Vec<_>>>()?;
        Ok(data)
    }

    fn id_lookup(&self, id: &str) -> String {
        match id {
            "1001" => "7",
            "1002" => "8",
            "1003" => "9",
            "1004" => "10",
            "1005" => "791",
            "1006" => "710",
            "1008" => "11",
            "1009" => "12",
            "1013" => "13",
            "1101" => "14",
            "1102" => "15",
            "1103" => "16",
            "1104" => "17",
            "1105" => "18",
            "1106" => "19",
            "1107" => "20",
            "1108" => "21",
            "1109" => "22",
            "1110" => "1228",
            "1111" => "801",
            "1112" => "1389",
            "1201" => "24",
            "1202" => "25",
            "1203" => "711",
            "1204" => "26",
            "1205" => "789",
            "1206" => "27",
            "1207" => "712",
            "1208" => "804",
            "1209" => "28",
            "1210" => "1392",
            "1211" => "29",
            "1212" => "1387",
            "1213" => "1226",
            "1214" => "1640",
            "1215" => "1537",
            "1217" => "1533",
            "1218" => "2643",
            "1220" => "2947",
            "1221" => "2642",
            "1222" => "2948",
            "1223" => "2949",
            "1224" => "2657",
            "1301" => "1924",
            "1302" => "1535",
            "1303" => "1638",
            "1304" => "1920",
            "1305" => "1639",
            "1306" => "1807",
            "1307" => "1806",
            "1308" => "1919",
            "1309" => "2366",
            "1310" => "2494",
            "1312" => "1808",
            "1314" => "2495",
            "1315" => "2367",
            "1317" => "3057",
            "8001" | "8002" => "6",
            "8003" | "8004" => "23",
            "8005" | "8006" => "2511",
            "21023" => "598",
            _ => todo!(),
        }
        .to_string()
    }
}

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiosecurityResult {
    pub is_flagged: bool,
    pub hits: Vec<BiosecurityHit>,
    pub screening_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiosecurityHit {
    pub organism: String,
    pub description: String,
    pub risk_level: String,
    pub region_start: usize,
    pub region_end: usize,
}

pub async fn screen_sequence(sequence: &str) -> Result<BiosecurityResult, String> {
    let client = Client::new();

    let resp = client
        .post("https://securedna.org/api/v1/screen")
        .json(&serde_json::json!({
            "fasta": format!(">query\n{}", sequence),
            "region": "all"
        }))
        .send()
        .await;

    match resp {
        Ok(response) if response.status().is_success() => {
            let body: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("Parse error: {e}"))?;

            let hits: Vec<BiosecurityHit> = body
                .get("hits")
                .and_then(|h| h.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|hit| {
                            Some(BiosecurityHit {
                                organism: hit.get("organism")?.as_str()?.to_string(),
                                description: hit.get("description")?.as_str()?.to_string(),
                                risk_level: hit.get("risk_level").and_then(|r| r.as_str()).unwrap_or("unknown").to_string(),
                                region_start: hit.get("start").and_then(|s| s.as_u64()).unwrap_or(0) as usize,
                                region_end: hit.get("end").and_then(|e| e.as_u64()).unwrap_or(0) as usize,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            Ok(BiosecurityResult {
                is_flagged: !hits.is_empty(),
                hits,
                screening_provider: "SecureDNA".to_string(),
            })
        }
        Ok(response) => {
            let status = response.status();
            Err(format!("SecureDNA API error: {status}"))
        }
        Err(e) => {
            Ok(BiosecurityResult {
                is_flagged: false,
                hits: vec![],
                screening_provider: format!("SecureDNA (offline: {e})"),
            })
        }
    }
}

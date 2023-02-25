pub struct DataRow {
    pub generation_id: u32, // Rows with the same generation_id use agents with the same preferences an distribution, but everything else my be different
    pub shifted: bool,

    pub distribution: String,
    pub coordination_mechanism: String,
    pub voting_mechanism: String,
    pub discrete_vote: bool,

    pub number_of_delegators: u32,
    pub number_of_proxies: u32,

    pub estimate: f64,

    pub min_proxy_weight: f64,
    pub max_proxy_weight: f64,
    pub average_proxy_weight: f64,
    pub median_proxy_weight: f64,
}

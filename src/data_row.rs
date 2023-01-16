pub struct DataRow {
    pub distribution: String,
    pub voting_mechanism: String,
    pub number_of_proxies: u32,
    pub number_of_delegates: u32,
    pub estimate: f64,
    pub min_proxy_weight: f64,
    pub max_proxy_weight: f64,
    pub average_proxy_weight: f64,
}

/*
 * model
 *     A collection of structs, that model the application.
 */
pub mod model
{
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Mapping
    {
        pub records: Vec<Account>,
    }
    
    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Account
    {
        pub name: String,
        pub assets: Vec<Asset>,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Asset
    {
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Configuration
    {
        pub range1_low: f64,
        pub range1_high: f64,
        pub range2_low: f64,
        pub range2_high: f64,
        pub range3_low: f64,
        pub range3_high: f64,
        pub range4_low: f64,
        pub range4_high: f64,
    }
}

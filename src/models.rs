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
}

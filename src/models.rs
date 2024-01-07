use serde::Deserialize;
use serde::Serialize;

use std::fmt::{self, Display};
use std::iter::FromIterator;
use std::marker::PhantomData as Phantom;
use std::str::FromStr;

use serde::de::{self, Deserializer, Visitor};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub code: String,
    pub product: Product,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "_id")]
    pub id: String,
    // #[serde(rename = "_keywords")]
    // pub keywords: Vec<String>,
    pub entry_dates_tags: Option<Vec<String>>,
    #[serde(rename = "expiration_date")]
    pub expiration_date: String,

    #[serde(rename = "product_name_en")]
    pub product_name_en: String,

    #[serde(rename = "quantity")]
    pub quantity: String,

    #[serde(rename = "brands", deserialize_with = "comma_separated")]
    pub brands: Vec<String>,

    pub nutriments: Nutriments,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutriments {
    pub carbohydrates: f64,
    #[serde(rename = "carbohydrates_100g")]
    pub carbohydrates_100g: f64,
    #[serde(rename = "carbohydrates_unit")]
    pub carbohydrates_unit: String,
    #[serde(rename = "carbohydrates_value")]
    pub carbohydrates_value: f64,
    pub energy: i64,
    #[serde(rename = "energy-kcal")]
    pub energy_kcal: i64,
    #[serde(rename = "energy-kcal_100g")]
    pub energy_kcal_100g: i64,
    #[serde(rename = "energy-kcal_unit")]
    pub energy_kcal_unit: String,
    // #[serde(rename = "energy-kcal_value")]
    // pub energy_kcal_value: i64,
    // #[serde(rename = "energy-kcal_value_computed")]
    // pub energy_kcal_value_computed: f64,
    // #[serde(rename = "energy-kj")]
    // pub energy_kj: i64,
    // #[serde(rename = "energy-kj_100g")]
    // pub energy_kj_100g: i64,
    // #[serde(rename = "energy-kj_unit")]
    // pub energy_kj_unit: String,
    // #[serde(rename = "energy-kj_value")]
    // pub energy_kj_value: i64,
    // #[serde(rename = "energy-kj_value_computed")]
    // pub energy_kj_value_computed: f64,
    // #[serde(rename = "energy_100g")]
    // pub energy_100g: i64,
    // #[serde(rename = "energy_unit")]
    // pub energy_unit: String,
    // #[serde(rename = "energy_value")]
    // pub energy_value: i64,
    // pub fat: f64,
    // #[serde(rename = "fat_100g")]
    // pub fat_100g: f64,
    // #[serde(rename = "fat_unit")]
    // pub fat_unit: String,
    // #[serde(rename = "fat_value")]
    // pub fat_value: f64,
    // pub fiber: i64,
    // #[serde(rename = "fiber_100g")]
    // pub fiber_100g: i64,
    // #[serde(rename = "fiber_unit")]
    // pub fiber_unit: String,
    // #[serde(rename = "fiber_value")]
    // pub fiber_value: i64,
    // #[serde(rename = "fruits-vegetables-legumes-estimate-from-ingredients_100g")]
    // pub fruits_vegetables_legumes_estimate_from_ingredients_100g: f64,
    // #[serde(rename = "fruits-vegetables-legumes-estimate-from-ingredients_serving")]
    // pub fruits_vegetables_legumes_estimate_from_ingredients_serving: f64,
    // #[serde(rename = "fruits-vegetables-nuts-estimate-from-ingredients_100g")]
    // pub fruits_vegetables_nuts_estimate_from_ingredients_100g: f64,
    // #[serde(rename = "fruits-vegetables-nuts-estimate-from-ingredients_serving")]
    // pub fruits_vegetables_nuts_estimate_from_ingredients_serving: f64,
    // #[serde(rename = "nova-group")]
    // pub nova_group: i64,
    // #[serde(rename = "nova-group_100g")]
    // pub nova_group_100g: i64,
    // #[serde(rename = "nova-group_serving")]
    // pub nova_group_serving: i64,
    // #[serde(rename = "nutrition-score-fr")]
    // pub nutrition_score_fr: i64,
    // #[serde(rename = "nutrition-score-fr_100g")]
    // pub nutrition_score_fr_100g: i64,
    // pub proteins: f64,
    // #[serde(rename = "proteins_100g")]
    // pub proteins_100g: f64,
    // #[serde(rename = "proteins_unit")]
    // pub proteins_unit: String,
    // #[serde(rename = "proteins_value")]
    // pub proteins_value: f64,
    // pub salt: f64,
    // #[serde(rename = "salt_100g")]
    // pub salt_100g: f64,
    // #[serde(rename = "salt_unit")]
    // pub salt_unit: String,
    // #[serde(rename = "salt_value")]
    // pub salt_value: f64,
    // #[serde(rename = "saturated-fat")]
    // pub saturated_fat: f64,
    // #[serde(rename = "saturated-fat_100g")]
    // pub saturated_fat_100g: f64,
    // #[serde(rename = "saturated-fat_unit")]
    // pub saturated_fat_unit: String,
    // #[serde(rename = "saturated-fat_value")]
    // pub saturated_fat_value: f64,
    // pub sodium: f64,
    // #[serde(rename = "sodium_100g")]
    // pub sodium_100g: f64,
    // #[serde(rename = "sodium_unit")]
    // pub sodium_unit: String,
    // #[serde(rename = "sodium_value")]
    // pub sodium_value: f64,
    // pub sugars: f64,
    // #[serde(rename = "sugars_100g")]
    // pub sugars_100g: f64,
    // #[serde(rename = "sugars_unit")]
    // pub sugars_unit: String,
    // #[serde(rename = "sugars_value")]
    // pub sugars_value: f64,
}

// stolen from https://github.com/serde-rs/serde/issues/581
fn comma_separated<'de, V, T, D>(deserializer: D) -> Result<V, D::Error>
where
    V: FromIterator<T>,
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    struct CommaSeparated<V, T>(Phantom<V>, Phantom<T>);

    impl<'de, V, T> Visitor<'de> for CommaSeparated<V, T>
    where
        V: FromIterator<T>,
        T: FromStr,
        T::Err: Display,
    {
        type Value = V;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string containing comma-separated elements")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let iter = s.split(",").map(FromStr::from_str);
            Result::from_iter(iter).map_err(de::Error::custom)
        }
    }

    let visitor = CommaSeparated(Phantom, Phantom);
    deserializer.deserialize_str(visitor)
}

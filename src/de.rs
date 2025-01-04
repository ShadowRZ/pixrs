//! Deserialize functions.

use std::{collections::HashMap, fmt::Display, hash::Hash, str::FromStr};

use serde::{Deserialize, Deserializer};

pub(crate) fn dict_value_to_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum HashMapOrVec<T> {
        HashMap(HashMap<String, T>),
        Vec(Vec<T>),
    }

    match HashMapOrVec::<T>::deserialize(deserializer)? {
        HashMapOrVec::HashMap(map) => Ok(map.into_values().collect()),
        HashMapOrVec::Vec(vec) => Ok(vec),
    }
}

pub(crate) fn dict_key_to_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Eq + Hash + FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    use serde::de::IgnoredAny;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum HashMapOrVec {
        HashMap(HashMap<String, IgnoredAny>),
        Vec(Vec<String>),
    }

    match HashMapOrVec::deserialize(deserializer)? {
        HashMapOrVec::HashMap(map) => {
            let keys = map.into_keys();
            let res: Result<Vec<T>, <T as FromStr>::Err> =
                keys.map(|key| <T as FromStr>::from_str(&key)).collect();
            res.map_err(serde::de::Error::custom)
        }
        HashMapOrVec::Vec(set) => {
            let keys = set.into_iter();
            let res: Result<Vec<T>, <T as FromStr>::Err> =
                keys.map(|key| <T as FromStr>::from_str(&key)).collect();
            res.map_err(serde::de::Error::custom)
        }
    }
}

pub(crate) fn deserialize_err_is_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(T::deserialize(deserializer).ok())
}

pub(crate) fn false_is_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrT<T> {
        #[allow(dead_code)]
        Bool(bool),
        T(T),
    }

    match BoolOrT::<T>::deserialize(deserializer)? {
        BoolOrT::Bool(_) => Ok(None),
        BoolOrT::T(val) => Ok(Some(val)),
    }
}

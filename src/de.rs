//! Deserialize functions.

use std::{collections::HashMap, hash::Hash};

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
    T: Eq + Hash + Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum HashMapOrVec<T>
    where
        T: Eq + Hash,
    {
        HashMap(HashMap<T, ()>),
        Vec(Vec<T>),
    }

    match HashMapOrVec::<T>::deserialize(deserializer)? {
        HashMapOrVec::HashMap(map) => Ok(map.into_keys().collect()),
        HashMapOrVec::Vec(set) => Ok(set),
    }
}

pub(crate) fn deserialize_err_is_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(T::deserialize(deserializer).ok())
}

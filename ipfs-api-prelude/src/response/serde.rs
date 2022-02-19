// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::serde::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use multibase::decode;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::marker::PhantomData;

pub struct IntegerVisitor;

impl<'de> Visitor<'de> for IntegerVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("integer")
    }

    fn visit_i8<E>(self, num: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(num.into())
    }

    fn visit_i32<E>(self, num: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(num.into())
    }

    fn visit_i64<E>(self, num: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(num)
    }

    fn visit_u8<E>(self, num: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(num.into())
    }

    fn visit_u32<E>(self, num: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(num.into())
    }

    fn visit_u64<E>(self, num: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(num as i64)
    }
}

/// Deserializes a sequence or null values as a vec.
///
pub fn deserialize_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    // Visits a sequence or null type, returning either the sequence
    // or an empty vector.
    //
    struct VecVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for VecVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("sequence or unit")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::new();

            while let Some(item) = seq.next_element()? {
                vec.push(item);
            }

            Ok(vec)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Default::default())
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(VecVisitor(PhantomData))
        }
    }

    deserializer.deserialize_option(VecVisitor(PhantomData))
}

/// Deserializes a map or null values as a HashMap.
///
pub fn deserialize_hashmap<'de, T, D>(deserializer: D) -> Result<HashMap<String, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    // Visits a map or null type, returning either the mapping
    // or an empty HashMap.
    //
    struct MapVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for MapVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = HashMap<String, T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("map or unit")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut hashmap = HashMap::new();

            while let Some((key, value)) = map.next_entry()? {
                hashmap.insert(key, value);
            }

            Ok(hashmap)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Default::default())
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(MapVisitor(PhantomData))
        }
    }

    deserializer.deserialize_option(MapVisitor(PhantomData))
}

pub fn deserialize_data_field<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;

    let (_, data) = decode(data).map_err(Error::custom)?;

    Ok(data)
}

pub fn deserialize_seqno_field<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let seqno: &str = Deserialize::deserialize(deserializer)?;

    let (_, seqno) = decode(seqno).map_err(Error::custom)?;
    let seqno = read_be_u64(&mut seqno.as_ref());

    Ok(seqno)
}

fn read_be_u64(input: &mut &[u8]) -> u64 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
    *input = rest;
    u64::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn deserialize_topic_field<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut topics: Vec<String> = Deserialize::deserialize(deserializer)?;

    for topic in topics.iter_mut() {
        let (_, decoded) = decode(&topic).map_err(Error::custom)?;

        let new_topic = String::from_utf8(decoded).map_err(Error::custom)?;

        *topic = new_topic;
    }

    Ok(topics)
}

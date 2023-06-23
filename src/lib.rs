#![feature(unboxed_closures)]
#![feature(associated_type_bounds)]
#![feature(iter_array_chunks)]

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::ops::Fn;

pub trait ArrayParMap<From, const SIZE: usize>: Send
{
    fn par_map<Map>(self, map: &Map) -> [Map::Output; SIZE]
    where
        Map: Fn<(From,), Output: Send> + Send + Sync;
}

impl<From, const SIZE: usize> ArrayParMap<From, SIZE> for [From; SIZE]
where
    From: Send
{
    fn par_map<Map>(self, map: &Map) -> [Map::Output; SIZE]
    where
        Map: Fn<(From,), Output: Send> + Send + Sync
    {
        self.into_par_iter()
            .map(map)
            .collect::<Vec<Map::Output>>()
            .into_iter()
            .array_chunks()
            .next()
            .unwrap()
    }
}
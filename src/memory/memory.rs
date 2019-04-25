use std::ops::{Index, IndexMut};

pub trait Accessable {
    type Address;
    type Data;

    fn get(&self, pos: Self::Address) -> Option<&Self::Data>;
    fn set(&mut self, pos: Self::Address, data: Self::Data) -> Result<(), std::string::String>;
}

pub struct Memory<I, C>
where
    I: Accessable,
    C: Accessable,
{
    internal_memory: I,
    cache: C,
}

struct CacheMemory<T, U> {
    data: T,
    tag: U,
}

pub struct CacheDirect<T> {
    internal_memory: Vec<CacheMemory<T, usize>>, //TODO Use something more generic
    cache_size: u32,
    line_size: u32,
    address_size: u32,
}

impl<T> Accessable for CacheDirect<T> {
    type Address = u32;
    type Data = T;

    fn get(&self, pos: Self::Address) -> Option<&Self::Data> {
        let cache_pos = self.calc_line_mask() & pos % (self.cache_positions() / self.line_size);
        let cache_data = &self.internal_memory[cache_pos as usize];
        if cache_data.tag as u32 == pos & self.calc_tag_mask() {
            Some(&cache_data.data)
        } else {
            None
        }
    }

    fn set(&mut self, pos: Self::Address, data: Self::Data) -> Result<(), std::string::String> {
        Ok(())
    }
}

impl<T> CacheDirect<T> {
    fn calc_tag_mask(&self) -> u32 {
        &self.address_size ^ (self.calc_line_mask() | self.calc_word_mask())
    }

    fn calc_line_mask(&self) -> u32 {
        self.cache_positions() << num_bits(self.calc_word_mask())
    }

    fn calc_word_mask(&self) -> u32 {
        self.line_size - 1
    }

    fn cache_positions(&self) -> u32 {
        self.cache_size / 8
    }
}

fn type_bits<T>() -> u32 {
    (std::mem::size_of::<T>() as u32) * 8
}

fn num_bits(x: u32) -> u32 {
    (type_bits::<u32>() as u32) - (x.leading_zeros() as u32) - 1
}

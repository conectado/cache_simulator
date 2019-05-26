use super::*;
use rand::distributions::{Distribution, Uniform};

pub struct CacheAssociative<T, U = u32>
where
    T: std::clone::Clone,
{
    pub internal_memory: Vec<CacheMemory<T, u32>>,
    pub cache_size: U,
    pub line_size: U,
    pub address_size: U,
}

impl<T> CacheAssociative<T>
where
    T: std::clone::Clone,
{
    pub fn calc_tag_mask(&self) -> u32 {
        (!0 >> (type_bits::<u32>() - &self.address_size)) ^ (self.calc_word_mask())
    }

    pub fn calc_word_mask(&self) -> u32 {
        self.line_size - 1
    }

    pub fn cache_positions(&self) -> u32 {
        self.cache_size / 8
    }
}

impl<T> Accessable for CacheAssociative<T>
where
    T: std::clone::Clone,
{
    type Address = u32;
    type Data = T;

    fn get(&mut self, pos: Self::Address) -> Option<Self::Data> {
        let data_tag = (self.calc_tag_mask() & pos) >> self.calc_tag_mask().trailing_zeros();
        let mut data = None;

        for cache_data in &self.internal_memory {
            if cache_data.tag == data_tag {
                data = Some(cache_data.data.clone())
            }
        }

        data
    }

    fn set(&mut self, pos: Self::Address, data: Self::Data) -> Result<(), std::string::String> {
        let data_tag = (self.calc_tag_mask() & pos) >> self.calc_tag_mask().trailing_zeros();

        let data = CacheMemory {
            data: data,
            tag: data_tag,
        };

        let between = Uniform::from(0..self.cache_positions() - 1);
        let mut rng = rand::thread_rng();
        self.internal_memory[between.sample(&mut rng) as usize] = data;
        Ok(())
    }
}

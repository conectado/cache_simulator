use super::*;

pub struct CacheDirect<T, U = u32>
where
    T: std::clone::Clone,
{
    pub internal_memory: Vec<CacheMemory<T, u32>>,
    pub cache_size: U,
    pub line_size: U,
    pub address_size: U,
}

impl<T> Accessable for CacheDirect<T>
where
    T: std::clone::Clone,
{
    type Address = u32;
    type Data = T;

    fn get(&mut self, pos: Self::Address) -> Option<Self::Data> {
        let cache_pos = ((self.calc_line_mask() & pos) >> self.calc_line_mask().trailing_zeros())
            % (self.cache_positions() / self.line_size);
        let cache_data = &self.internal_memory[(cache_pos | pos & self.calc_word_mask()) as usize];
        if cache_data.tag as u32
            == (pos & self.calc_tag_mask()) >> self.calc_tag_mask().trailing_zeros()
        {
            Some(cache_data.data.clone())
        } else {
            None
        }
    }

    fn set(&mut self, pos: Self::Address, data: Self::Data) -> Result<(), std::string::String> {
        let cache_pos = ((self.calc_line_mask() & pos) >> self.calc_line_mask().trailing_zeros())
            % (self.cache_positions() / self.line_size);

        let data = CacheMemory {
            data: data,
            tag: (pos & self.calc_tag_mask()) >> self.calc_tag_mask().trailing_zeros(),
        };

        self.internal_memory[cache_pos as usize] = data;
        Ok(())
    }
}

impl<T> CacheDirect<T>
where
    T: std::clone::Clone,
{
    pub fn calc_tag_mask(&self) -> u32 {
        (!0 >> (type_bits::<u32>() - &self.address_size))
            ^ (self.calc_line_mask() | self.calc_word_mask())
    }

    pub fn calc_line_mask(&self) -> u32 {
        (self.cache_positions() / self.line_size - 1) << num_bits(self.calc_word_mask())
    }

    pub fn calc_word_mask(&self) -> u32 {
        self.line_size - 1
    }

    pub fn cache_positions(&self) -> u32 {
        self.cache_size / 8
    }
}

// TODO generalize types
pub trait Accessable {
    type Address;
    type Data: std::clone::Clone;

    fn get(&mut self, pos: Self::Address) -> Option<Self::Data>;
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

impl<I, C> Accessable for Memory<I, C>
where
    I: Accessable<Address = u32, Data = u32>,
    C: Accessable<Address = u32, Data = u32>,
{
    type Address = u32;
    type Data = u32;

    fn get(&mut self, pos: Self::Address) -> Option<Self::Data> {
        match self.cache.get(pos) {
            Some(data) => Some(data),
            None => {
                if let Some(res) = self.internal_memory.get(pos) {
                    self.cache.set(pos, res.clone()).unwrap();
                    Some(res)
                } else {
                    None
                }
            }
        }
    }

    fn set(&mut self, pos: Self::Address, data: Self::Data) -> Result<(), std::string::String> {
        self.internal_memory.set(pos, data).unwrap();
        Ok(())
    }
}

struct CacheMemory<T, U>
where
    T: std::clone::Clone,
{
    data: T,
    tag: U,
}

pub struct RAM<T>
where
    T: std::clone::Clone,
{
    internal_memory: Vec<T>,
    mem_size: u32,
}

impl<T> Accessable for RAM<T>
where
    T: std::clone::Clone,
{
    type Address = u32;
    type Data = T;

    fn get(&mut self, pos: Self::Address) -> Option<Self::Data> {
        Some(self.internal_memory[pos as usize].clone())
    }

    fn set(&mut self, pos: Self::Address, data: Self::Data) -> Result<(), std::string::String> {
        self.internal_memory[pos as usize] = data;
        Ok(())
    }
}

pub struct CacheDirect<T, U = u32>
where
    T: std::clone::Clone,
{
    internal_memory: Vec<CacheMemory<T, u32>>,
    cache_size: U,
    line_size: U,
    address_size: U,
}

impl<T, U> Accessable for CacheDirect<T, U>
where
    T: std::clone::Clone,
{
    type Address = U;
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

impl<T, U> CacheDirect<T, U>
where
    T: std::clone::Clone,
{
    fn calc_tag_mask(&self) -> u32 {
        (!0 >> (type_bits::<u32>() - &self.address_size))
            ^ (self.calc_line_mask() | self.calc_word_mask())
    }

    fn calc_line_mask(&self) -> u32 {
        (self.cache_positions() / self.line_size - 1) << num_bits(self.calc_word_mask())
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
    (type_bits::<u32>() as u32) - (x.leading_zeros() as u32)
}

#[cfg(test)]
pub mod tests;

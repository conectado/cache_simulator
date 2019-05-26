// TODO generalize types
use rand::distributions::{Distribution, Uniform};

pub mod associative;
pub mod direct;

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
    line_size: u32,
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
            // TODO set all words in the line
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

pub struct CacheMemory<T, U>
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

fn type_bits<T>() -> u32 {
    (std::mem::size_of::<T>() as u32) * 8
}

fn num_bits(x: u32) -> u32 {
    (type_bits::<u32>() as u32) - (x.leading_zeros() as u32)
}

#[cfg(test)]
pub mod tests;

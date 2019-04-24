use std::ops::{Index, IndexMut};

pub struct Memory<I, C>
where
    I: Index<usize> + IndexMut<usize>,
    C: Index<usize> + IndexMut<usize>,
{
    internal_memory: I,
    cache: C,
}

struct CacheDirect<T> {
    internal_memory: Vec<T>,
    cache_size: usize,
    line_size: usize,
    address_size: usize,
}

impl<'a, T> Index<usize> for &'a CacheDirect<T> {
    type Output = Option<&'a T>;
    fn index(&'a self, i: usize) -> &'a Self::Output {
        &Some(&self.internal_memory[i])
    }
}

impl<'a, T> IndexMut<usize> for &'a CacheDirect<T> {
    fn index_mut(&'a mut self, i: usize) -> &'a mut Self::Output {
        &mut Some(&self.internal_memory[i])
    }
}

impl<T> CacheDirect<T> {
    fn calc_tag_mask(&self) -> usize {
        &self.address_size ^ (self.calc_line_mask() | self.calc_word_mask())
    }

    fn calc_line_mask(&self) -> usize {
        self.cache_positions() << num_bits(self.calc_word_mask())
    }

    fn calc_word_mask(&self) -> usize {
        self.line_size - 1
    }

    fn cache_positions(&self) -> usize {
        self.cache_size / 8
    }
}

fn type_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

fn num_bits(x: usize) -> usize {
    (type_bits::<usize>() as usize) - (x.leading_zeros() as usize) - 1
}

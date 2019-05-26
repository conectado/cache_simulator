use super::*;
use associative::*;
use direct::*;
#[test]
fn correct_tag_mask() {
    let mem_test: CacheDirect<u32> = CacheDirect {
        internal_memory: vec![],
        cache_size: 524288,
        line_size: 4,
        address_size: 24,
    };

    assert!(mem_test.calc_tag_mask() == 0xFF0000u32);
}

#[test]
fn correct_word_mask() {
    let mem_test: CacheDirect<u32> = CacheDirect {
        internal_memory: vec![],
        cache_size: 524288,
        line_size: 4,
        address_size: 24,
    };

    assert!(mem_test.calc_word_mask() == 0x3u32);
}

#[test]
fn correct_line_mask() {
    let mem_test: CacheDirect<u32> = CacheDirect {
        internal_memory: vec![],
        cache_size: 524288,
        line_size: 4,
        address_size: 24,
    };

    assert!(mem_test.calc_line_mask() == 0xFFFCu32);
}

#[test]
fn correct_get() {
    let mut mem_test: CacheDirect<u32> = CacheDirect {
        internal_memory: vec![
            CacheMemory {
                data: 1,
                tag: 0b1u32,
            },
            CacheMemory {
                data: 2,
                tag: 0b0u32,
            },
        ],
        cache_size: 16,
        line_size: 1,
        address_size: 2,
    };

    if let Some(x) = mem_test.get(0b10) {
        assert!(x == 1);
    } else {
        assert!(false);
    }

    assert!(mem_test.get(0b00).is_none());

    if let Some(x) = mem_test.get(0b01) {
        assert!(x == 2);
    } else {
        assert!(false);
    }

    assert!(mem_test.get(0b11).is_none());
}

#[test]
fn correct_set() {
    let mut mem_test: CacheDirect<u32> = CacheDirect {
        internal_memory: vec![
            CacheMemory { tag: 0, data: 0 },
            CacheMemory { tag: 0, data: 0 },
        ],
        cache_size: 16,
        line_size: 1,
        address_size: 2,
    };

    mem_test.set(0b10, 1);
    mem_test.set(0b01, 2);
    if let Some(x) = mem_test.get(0b10) {
        assert!(x == 1);
    } else {
        assert!(false);
    }

    assert!(mem_test.get(0b00).is_none());

    if let Some(x) = mem_test.get(0b01) {
        assert!(x == 2);
    } else {
        assert!(false);
    }

    assert!(mem_test.get(0b11).is_none());
}

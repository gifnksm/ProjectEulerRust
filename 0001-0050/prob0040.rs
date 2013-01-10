extern mod euler;
use euler::calc::{ num_to_digits };
use core::util;

// d_(a*10 + b)
//    0 1 2 3 4 5 6 7 8 9
// 0  - 1 2 3 4 5 6 7 8 9
// 1  1 0 1 1 1 2 1 3 1 4
// 2  1 5 1 6 1 7 1 8 1 9
// 3  2 0 2 1 2 2 2 3 2 4
// 4  2 5 2 6 2 7 2 8 2 9
// 5  3 0 3 1 3 2 3 3 3 4
//
// [1,   9]   => 9
// [10,  99]  => 90
// [100, 999] => 900
//
// num => idx
// 1 <= n <= 9       => i = n
// 10 <= n <= 99     => i = 2 * (n - 10) + 10 = 2n - 10
// 100 <= n <= 999   => i = 3 * (n - 100) + 2 * 100 - 10 = 3n - 110
// 1000 <= n <= 9999 => i = 4 * (n - 1000) + 3 * 1000 - 110 = 4n - 1110
//
struct Area {
    num_digit: uint,
    min_val: uint,
    max_val: uint,
    min_idx: uint,
    max_idx: uint
}

struct IdxValueMap {
    priv area: ~[Area]
}

fn IdxValueMap() -> IdxValueMap {
    return IdxValueMap { area: ~[ Area {
        num_digit: 0,
        min_val: 0,
        max_val: 0,
        min_idx: 0,
        max_idx: 0
    } ] };
}

impl IdxValueMap {
    priv fn extend(&mut self) {
        let last = self.area.last();
        let num_digit = last.num_digit + 1;
        let min_val = last.max_val + 1;
        let max_val = min_val * 10 - 1;
        let min_idx = last.max_idx + 1;
        let max_idx = last.max_idx + (max_val - min_val + 1) * num_digit;
        self.area.push(Area {
            num_digit: num_digit,
            min_val: min_val, max_val: max_val,
            min_idx: min_idx, max_idx: max_idx
        });
    }

    priv fn each_area(&mut self, f: fn(Area) -> bool) {
        for uint::range(0, self.area.len()) |i| {
            if !f(self.area[i]) { return; }
        }
        loop {
            self.extend();
            if !f(self.area.last()) { return; }
        }
    }

    priv fn get_area_by_idx(&mut self, idx: uint) -> Area {
        for self.each_area |area| {
            if area.min_idx <= idx && idx <= area.max_idx {
                return area;
            }
        }
        util::unreachable();
    }

    pub fn get_value_by_idx(&mut self, idx: uint) -> uint {
        let area = self.get_area_by_idx(idx);
        return area.min_val + ((idx - area.min_idx) / area.num_digit);
   }
    pub fn get_digit_by_idx(&mut self, idx: uint) -> uint {
        let area = self.get_area_by_idx(idx);
        let val  = area.min_val + ((idx - area.min_idx) / area.num_digit);
        return num_to_digits(val, 10)[(idx - area.min_idx) % area.num_digit];
    }
}


fn main() {
    let mut map = IdxValueMap();
    let idx = &[ 1, 10, 100, 1000, 10000, 100000, 1000000 ];
    let mut prod = 1;
    for idx.each |i| {
        prod *= map.get_digit_by_idx(*i);
    }
    io::println(fmt!("answer: %u", prod));
}

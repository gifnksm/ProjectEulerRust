extern mod euler;
use euler::calc::{ get_gcd };

// AB / AC => NG (10A+B : 10A+C = B : C => 10AC+BC = 10AB+BC => 10A(C-B) = 0 -> trivial)
// BA / CA => NG
// AB / CA => (10A + B : 10C + A = B : C => 10AC + BC = 10BC + AB => A(10C-B) = 9BC)
// BA / AC => (10B + A : 10A + C = B : C => 10BC + AC = 10AB + BC => A(10B-C) = 9BC)
//
// * AB / CA = B / C
// A = 9BC / (10C - B)
// C > B
//
// * BA / AC = B / C
// A = 9BC / (10B - C)
// C > B

fn main() {
    let mut prod_numer = 1;
    let mut prod_denom = 1;

    for uint::range(1, 10) |b| {
        for uint::range(b + 1, 10) |c| {
            let a_numer = 9 * b * c;
            let a_denom = 10 * c - b;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                let a = a_numer / a_denom;
                prod_numer *= b;
                prod_denom *= c;
                io::println(fmt!("%u%u/%u%u = %u/%u", a, b, c, a, b, c));
            }
        }
    }
    for uint::range(1, 10) |b| {
        for uint::range(b + 1, 10) |c| {
            let a_numer = 9 * b * c;
            let a_denom = 10 * b - c;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                let a = a_numer / a_denom;
                prod_numer *= b;
                prod_denom *= c;
                io::println(fmt!("%u%u/%u%u = %u/%u", b, a, a, c, b, c));
            }
        }
    }

    let gcd = get_gcd(prod_numer, prod_denom);
    io::println(fmt!("%u/%u", prod_numer / gcd, prod_denom / gcd));
    io::println(fmt!("answer: %u", prod_denom / gcd));
}

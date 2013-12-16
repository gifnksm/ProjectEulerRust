#[crate_type = "rlib"];

pub static EXPECTED_ANSWER: &'static str = "100";

// AB / AC => NG (10A+B : 10A+C = B : C => 10AC+BC = 10AB+BC => 10A(C-B) = 0 -> trivial)
// BA / CA => NG
// AB / CA => (10A + B : 10C + A = B : C => 10AC + BC = 10BC + AB => A(10C-B) = 9BC)
// BA / AC => (10B + A : 10A + C = B : C => 10BC + AC = 10AB + BC => A(10B-C) = 9BC)
//
// * Pattern 1: AB / CA = B / C
// A = 9BC / (10C - B)
// C > B
//
// * Pattern 2: BA / AC = B / C
// A = 9BC / (10B - C)
// C > B

pub fn solve() -> ~str {
    let mut prod_numer = 1;
    let mut prod_denom = 1;

    for b in range(1u, 10) {
        for c in range(b + 1, 10) {
            // Pattern 1
            let a_numer = 9 * b * c;
            let a_denom = 10 * c - b;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                prod_numer *= b;
                prod_denom *= c;
            }
            // Pattern 2
            let a_numer = 9 * b * c;
            let a_denom = 10 * b - c;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                prod_numer *= b;
                prod_denom *= c;
            }
        }
    }

    let gcd = prod_numer.gcd(&prod_denom);
    return (prod_denom / gcd).to_str();
}

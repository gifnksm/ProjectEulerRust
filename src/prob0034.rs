#[link(name = "prob0034", vers = "0.0")];
#[crate_type = "lib"];

pub static EXPECTED_ANSWER: &'static str = "40730";

pub fn solve() -> ~str {
    let mut facts: [uint, ..10] = [ 0, ..10 ];
    facts[0] = 1;
    for i in range(1, facts.len()) {
        facts[i] = facts[i - 1] * i;
    }

    let mut answer = 0;
    for n in range(0, facts[9].to_str().len() * facts[9]) {
        let mut itr = n;
        let mut sum = 0;
        while itr > 0 {
            sum += facts[itr % 10];
            itr /= 10;
        }
        if sum == n {
            answer += sum;
        }
    }

    return (answer - 1 - 2).to_str();
}

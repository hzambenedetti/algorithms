use crate::lcs::lcs;

mod lcs;

fn main() {
    
    let x = [1, 2, 3, 4, 5, 6 ,7];
    let y = [0, 1, 2, 3, 4, 5, 6 ,7];
    lcs(&x, &y);
}

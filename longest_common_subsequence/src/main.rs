use crate::lcs::lcs;

mod lcs;

fn main() {
    
    let x = ['A', 'B', 'C', 'B', 'D','A','B'];
    let y = ['B','D','C','A','B','A'];

    lcs(&x, &y);
}

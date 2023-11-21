use crate::discback::discback;
mod discback;


fn main() {
    let value = [3,2,4,4];
    let weight = [4,3,2,3];
    let max_weight = 6;

    discback(&value, &weight, max_weight);
}

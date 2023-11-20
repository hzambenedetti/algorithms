use std::cmp::Eq;
use std::fmt::Display;

pub fn lcs<T: Eq + Display>(x: &[T], y: &[T]){
  println!("x.len() == {}", x.len());
  println!("y.len() == {}", y.len());
  let c = lcs_length(&x, &y);
  lcs_print(&x, &y, &c, x.len(), y.len());
  println!();
}

fn lcs_length<T: Eq>(x: &[T], y: &[T]) -> Vec<Vec<u32>>{
    let mut common: Vec<Vec<u32>> = vec![vec![0; y.len() + 1] ; x.len() + 1];
    
    for i in 1..=x.len(){
        for j in 1..=y.len(){
            if x[i-1] == y[j-1]{
                common[i][j] = common[i-1][j-1] + 1;
            }
            else{
                common[i][j] = (common[i-1][j]).max(common[i][j-1]);
            }
        }
    }
    common
}

fn lcs_print<T: Eq + Display>(x: &[T], y: &[T], c: &Vec<Vec<u32>>, i: usize, j: usize){
    if i > 0 && j > 0{
        if x[i-1] == y[j-1]{
            lcs_print(x, y, c, i-1, j-1);
            print!("{} ", x[i -1]);
        }
        else if c[i-1][j] >= c[i][j-1]{
            lcs_print(x,y,c,i-1,j);
        }
        else{
            lcs_print(x,y,c,i,j-1);
        }
    }
}

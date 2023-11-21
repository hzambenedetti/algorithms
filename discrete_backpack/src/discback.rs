
pub fn discback(value: &[u32], weight: &[u32], max_weight: usize){
   let (max, obj) = disckback_calc(value, weight, max_weight);
   println!("max: {}", max[value.len()][max_weight]);
   disckback_print(&obj, value.len(), max_weight);
   println!();
   //dbg!(&obj);
}


fn disckback_calc(value: &[u32], weight: &[u32], max_weight: usize) -> (Vec<Vec<u32>>, Vec<Vec<usize>>){
    let mut max_value: Vec<Vec<u32>> = vec![vec![0; max_weight + 1]; value.len() + 1];
    let mut objects: Vec<Vec<usize>> = vec![vec![0; max_weight + 1]; value.len() + 1];
    for i in 1..=value.len(){
        for j in 1..=max_weight{
            if j < weight[i - 1] as usize{
                max_value[i][j] = max_value[i-1][j];
                objects[i][j] = j;
            }
            else if max_value[i-1][j] > max_value[i-1][j-weight[i-1]as usize] + value[i-1]{
                max_value[i][j] = max_value[i-1][j];
                objects[i][j] = j;
            }
            else{
                max_value[i][j] = max_value[i-1][j-weight[i-1]as usize] + value[i-1]; 
                objects[i][j] = j - weight[i-1] as usize;
            }
        }   
    }

    (max_value, objects)
}

fn disckback_print(obj: &Vec<Vec<usize>>,n: usize, m: usize){
    if n != 0{
        if obj[n][m] == m{
            disckback_print(obj,n-1, m);
        }
        else{
            disckback_print(obj, n-1, obj[n][m]);
            print!("{n} ");
        }
    }
}

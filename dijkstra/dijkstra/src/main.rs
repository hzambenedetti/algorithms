use std::collections::binary_heap::BinaryHeap;

fn dijkstra(s: usize, graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize>{
    let mut d: Vec<usize> = vec![std::usize::MAX/2; graph.len()];
    
    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    pq.push((s, 0));
    while !pq.is_empty(){
        let (v, dv) = pq.pop().unwrap();
        if d[v] < dv{
            continue;
        }

        for &(u, du) in graph[v].iter(){
            if dv + du < d[u]{
                d[u] = dv + du;
            }
        }
    }
    return d;
}

fn main(){
    let x = 5;
    println!("{x}");
}


pub fn min_dist(dist: Vec<u32>,visited: Vec<bool>) -> u32 {
    let mut min: u32 = u32::MAX;
    let mut mindex = min;
    let N = dist.len();

    for i in 0..N {
        if dist[i as usize] <= min && visited[i as usize] == false {
            min = dist[i as usize];
            mindex = i as u32;
        }
    }

    mindex
}

pub fn shortest_path(graph: Vec<u32>,N: u32,s: u32 ) -> Vec<u32> {
    let mut visited = Vec::<bool>::new();
    let mut dist = Vec::<u32>::new();

    for i in 0..N {
        visited.push(false);
        dist.push(u32::MAX);
    }

    dist[s as usize] = 0;

    for i in 0..N-1 {
        let u: u32 = min_dist(dist.clone(),visited.clone());
        visited[u as usize] = true;

        for v in 0..N {
            let ind = u * N + v;
            if !visited[v as usize] && graph[ind as usize] != 0 && dist[u as usize] != u32::MAX && (dist[u as usize] + graph[ind as usize] < dist[v as usize]) {
                dist[v as usize] = dist[u as usize] + graph[ind as usize];
            }
        }
    }

    dist
}

fn main() {
    let N: u32 = 9;
    
    let graph: Vec<u32> = vec![0, 4, 0, 0, 0, 0, 0, 8, 0,4, 0, 8, 0, 0, 0, 0, 11, 0,0, 8, 0, 7, 0, 4, 0, 0, 2, 0, 0, 7, 0, 9, 14, 0, 0, 0,
    0, 0, 0, 9, 0, 10, 0, 0, 0, 0, 0, 4, 14, 10, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 6, 8, 11, 0, 0, 0, 0, 1, 0, 7, 0, 0, 2, 0, 0, 0, 6, 7, 0];

    println!("{:?}", graph);

    println!("SHORTEST PATH MATRIX");

    let mut dist: Vec<u32> = shortest_path(graph.clone(),N.clone(),0);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),1);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),2);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),3);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),4);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),5);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),6);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),7);

    println!("{:?}", dist);

    dist = shortest_path(graph.clone(),N.clone(),8);

    println!("{:?}", dist);

}

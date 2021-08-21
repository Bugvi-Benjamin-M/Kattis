use std::io;

type Graph = Vec<Vec<u8>>;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

// fn print_graph_debug(graph: &Graph) {
//     let mut graph_out = "".to_owned();
//     for i in 0..graph.len() {
//         for j in 0..graph.len() {
//             graph_out.push_str(&format!("{} ", graph[i][j]));
//         }
//         graph_out = graph_out.trim_end().to_string();
//         graph_out.push_str("\n");
//     }
//     print!("{}", graph_out);
// }

fn is_weak_vertex(graph: &Graph, i: usize) -> bool {
    
    let vertex: Vec<u8> = graph[i].to_owned();
    
    for j in 0..graph.len() {
        if vertex[j] == 1 {
            let next_vertex: Vec<u8> = graph[j].to_owned();
            for k in 0..graph.len() {
                if next_vertex[k] == 1 {
                    let next_next_vertex: Vec<u8> = graph[k].to_owned();
                    if next_next_vertex[i] == 1 { return false }
                }
            }
        }
    }
    true
}

fn main() {
    
    loop {
        
        let n: i8 = input().trim().parse().unwrap();
        if n == -1 { break; }
        
        let n_usize = n as usize;
        let mut weak_vertices: Vec<u8> = vec![];
        let mut graph: Graph = vec![vec![0; n_usize]; n_usize];
        
        for i in 0..n_usize {
            let row = input().trim().to_owned();
            let mut row_split = row.split_ascii_whitespace();
            for j in 0..n_usize {
                graph[i][j] = row_split.next().unwrap().parse().unwrap();
            }
        }
        
        //print_graph_debug(&graph);

        for i in 0..n_usize {
            if is_weak_vertex(&graph, i) { weak_vertices.push(i as u8) }
        }
        for i in 0..weak_vertices.len() {
            print!("{} ", weak_vertices[i])
        }
        if weak_vertices.len() > 0 { println!(); }
    }
}

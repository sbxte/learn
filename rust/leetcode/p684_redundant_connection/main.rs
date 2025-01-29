fn get_parent(p: &[i32], i: i32) -> i32 {
    if p[i as usize - 1] == i {
        return i;
    }
    get_parent(p, p[i as usize - 1])
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut parents: Vec<i32> = (1..=edges.len() as i32).collect();

    for edge in &edges {
        let edge = unsafe { (*edge.get_unchecked(0), *edge.get_unchecked(1)) };
        let p0 = get_parent(&parents, edge.0);
        let p1 = get_parent(&parents, edge.1);
        if p0 == p1 {
            return vec![edge.0, edge.1];
        }
        if p0 != edge.0 {
            parents[p1 as usize - 1] = parents[p0 as usize - 1];
        } else {
            parents[p0 as usize - 1] = parents[p1 as usize - 1];
        }
        dbg!(&parents);
    }
    vec![0, 0]
}

fn main() {
    println!("hi mom");
}

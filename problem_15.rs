use std::collections::HashMap;


pub fn main(size: i64) -> i64 {
    let mut nodes = HashMap::new();

    //nodes.insert((0, 0), size);

    // if nodes.contains_key(&(0, 0)) {
    //    return *nodes.get(&(0, 0)).unwrap();
    //}

    // return 0;
    return in_map(size, (0, 0), &mut nodes);
}

pub fn in_map(size: i64, position: (i64, i64), nodes: &mut HashMap<(i64, i64), i64>) -> i64 {
    /* match nodes.get(position) {
        Some(&node) => node,
        _ => 0,
    } */

    match nodes.get(&position) {
        Some(&node) => return node,
        None => {
            let result = spread(size, position, nodes);
            nodes.insert(position, result);
            return result;
        }
    }
}

pub fn spread(size: i64, position: (i64, i64), nodes: &mut HashMap<(i64, i64), i64>) -> i64 {
    let x = position.0;
    let y = position.1;
    if x > size - 1 || y > size - 1 {
        println!("Done");
        return 1;
    } else {
        return in_map(size, (x + 1, y), nodes) + in_map(size, (x, y + 1), nodes);

        //return spread(&Vector::new(x + 1.0, y)) + spread(&Vector::new(x, y + 1.0));
    }
}

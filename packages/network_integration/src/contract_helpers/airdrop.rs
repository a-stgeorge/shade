use cosmwasm_std::Binary;

pub fn proof_from_tree(indices: &Vec<usize>, tree: &Vec<Vec<[u8; 32]>>) -> Vec<Binary> {
    let mut current_indices: Vec<usize> = indices.clone();
    let mut helper_nodes: Vec<Binary> = Vec::new();

    for layer in tree {
        let mut siblings: Vec<usize> = Vec::new();
        let mut parents: Vec<usize> = Vec::new();

        for index in current_indices.iter() {
            if index % 2 == 0 {
                siblings.push(index + 1);
                parents.push(index / 2);
            } else {
                siblings.push(index - 1);
                parents.push((index - 1) / 2);
            }
        }

        for sibling in siblings {
            if !current_indices.contains(&sibling) {
                if let Some(item) = layer.get(sibling) {
                    helper_nodes.push(Binary(item.to_vec()));
                }
            }
        }

        parents.dedup();
        current_indices = parents.clone();
    }

    helper_nodes
}

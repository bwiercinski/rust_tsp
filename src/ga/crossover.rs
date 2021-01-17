use rand::Rng;

pub fn pmx_crossover<R: Rng>(a: &[usize], b: &[usize], rng: &mut R) -> (Vec<usize>, Vec<usize>) {
    let [split1, split2] = {
        let mut splits = [rng.gen_range(0..a.len()), rng.gen_range(0..a.len())];
        splits.sort();
        splits
    };
    let mut genotype_a = Vec::from(a);
    let mut genotype_b = Vec::from(b);

    let mut mapping_a = vec![None; genotype_a.len()];
    let mut mapping_b = vec![None; genotype_b.len()];
    genotype_a[split1..split2]
        .iter()
        .zip(genotype_b[split1..split2].iter())
        .for_each(|(gene_a, gene_b)| {
            mapping_a[*gene_b] = Some(*gene_a);
            mapping_b[*gene_a] = Some(*gene_b);
        });

    genotype_a[split1..split2].swap_with_slice(&mut genotype_b[split1..split2]);

    for i in (0..split1).chain(split2..a.len()) {
        if let Some(root) = find_root(&mapping_a, genotype_a[i]) {
            genotype_a[i] = root;
        }
        if let Some(root) = find_root(&mapping_b, genotype_b[i]) {
            genotype_b[i] = root;
        }
    }

    (genotype_a, genotype_b)
}

fn find_root(mapping: &[Option<usize>], start: usize) -> Option<usize> {
    let mut current = mapping[start];
    let mut last = None;
    while let Some(next) = current {
        last = Some(next);
        current = mapping[next];
    }
    last
}

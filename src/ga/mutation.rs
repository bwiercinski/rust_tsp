use rand::Rng;

pub fn swap_mutation<R: Rng>(a: &mut [usize], rate: f64, rng: &mut R) {
    for i in 0..a.len() {
        if rng.gen::<f64>() < rate {
            a.swap(i, rng.gen_range(0..a.len()))
        }
    }
}

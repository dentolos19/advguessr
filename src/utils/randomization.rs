use rand::Rng;

pub fn generate_random_number(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..(max + 1))
}
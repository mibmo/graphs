pub mod id;

pub mod vertex;
pub use vertex::Vertex;

fn main() {
    let mut verticies: Vec<_> = ["alice", "bob", "mallory", "rumle", "sigrid"]
        .into_iter()
        .map(Vertex::new)
        .collect();

    link_randomly(&mut verticies);
    dbg!(&verticies);

    let count = verticies
        .into_iter()
        .map(|vertex| vertex.edges().count())
        .sum::<usize>();
}

fn link_randomly<T>(verticies: &mut Vec<Vertex<T>>) {
    use rand::{rngs::SmallRng, seq::index::sample, Rng, SeedableRng};
    let mut rng = SmallRng::from_entropy();

    (0..5).for_each(|_| {
        let indicies = sample(&mut rng, verticies.len(), 2);
        let mut from = &verticies[indicies.index(0)];
        let mut to = &verticies[indicies.index(1)];

        if rng.gen_bool(0.5) {
            unsafe {
                std::ptr::swap(&mut from, &mut to);
            }
        }

        from.connect(to);
    });
}

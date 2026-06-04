use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random_action(actions: &Vec<(usize, u8)>) -> Option<(usize, u8)> {
    let mut rng = thread_rng();
    actions.choose(&mut rng).cloned()
}

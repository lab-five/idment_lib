use rand::{rng, seq::IndexedRandom};

pub struct Rand {
    charset: Vec<char>, // 字符集
}

impl Rand {
    pub fn new() -> Self {
        Rand {
            charset: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                .chars()
                .collect(),
        }
    }

    pub async fn get_rand(&self, length: usize) -> String {
        let mut rng = rng();
        (0..length)
            .map(|_| *self.charset.choose(&mut rng).unwrap())
            .collect()
    }

    pub async fn rand_string(length: usize) -> String {
        let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();
        let mut rng = rng();
        let random_string: String = (0..length)
            .map(|_| *charset.choose(&mut rng).unwrap())
            .collect();
        random_string
    }

    pub async fn rand_number(length: usize) -> i32 {
        let charset: Vec<char> = "0123456789".chars().collect();
        let mut rng = rng();
        let random: String = (1..length)
            .map(|_| *charset.choose(&mut rng).unwrap())
            .collect();
        random.parse().expect("Invalid number generated")
    }
}

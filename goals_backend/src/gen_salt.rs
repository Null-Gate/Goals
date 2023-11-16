use rand::{distributions::DistString, prelude::Distribution, rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct GenString {
    rngs: ThreadRng,
}

impl Distribution<u8> for GenString {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10;
        const GEN_ASCII_STR_CHARSET: &[u8] =
            r#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"#.as_bytes();
        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

impl DistString for GenString {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        unsafe {
            let v = string.as_mut_vec();
            v.extend(self.sample_iter(rng).take(len));
        }
    }
}

impl GenString {
    pub fn new() -> Self {
        Self {
            rngs: rand::thread_rng(),
        }
    }

    pub fn gen_string(&mut self, min: usize, max: usize) -> String {
        self.sample_string(
            &mut self.rngs.to_owned(),
            self.to_owned().rngs.gen_range(min..max),
        )
    }
}

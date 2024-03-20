use rand::{rngs::SmallRng, Rng, SeedableRng};

use std::fmt::{self, Debug, Display, Formatter, Write};

const ID_BYTES: usize = 4;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    inner: [u8; ID_BYTES],
}

impl Id {
    pub fn new() -> Self {
        let rng = SmallRng::from_entropy();
        Self::from_rng(rng)
    }

    pub fn from(inner: [u8; ID_BYTES]) -> Self {
        Self { inner }
    }

    pub fn from_seed<S: Into<<SmallRng as SeedableRng>::Seed>>(seed: S) -> Self {
        let rng = SmallRng::from_seed(seed.into());
        Self::from_rng(rng)
    }

    pub fn from_rng(mut rng: impl Rng) -> Self {
        let inner = (0..ID_BYTES)
            .map(|_| rng.gen())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { inner }
    }
}

impl AsRef<[u8; ID_BYTES]> for Id {
    fn as_ref(&self) -> &[u8; ID_BYTES] {
        &self.inner
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;
        for (i, byte) in self.inner.iter().enumerate() {
            let [c1, c2] = byte_to_hex(*byte);
            f.write_char(c1)?;
            f.write_char(c2)?;
            if i < ID_BYTES - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_char(']')
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&bytes_to_hex(&self.inner))
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().copied().map(byte_to_hex).flatten().collect()
}

fn byte_to_hex(n: u8) -> [char; 2] {
    let upper = n >> 4;
    let lower = n & 0xF;

    // SAFETY: values can only lie within valid ASCII ranges
    let to_hex = |n: u8| -> char {
        match n {
            0..=9 => unsafe { char::from_u32_unchecked(n as u32 + 48) },
            10..=15 => unsafe { char::from_u32_unchecked(n as u32 + 87) },
            _ => unreachable!("Number only in range 0..=15"),
        }
    };

    [to_hex(upper), to_hex(lower)]
}

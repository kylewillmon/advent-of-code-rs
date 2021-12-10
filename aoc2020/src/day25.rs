use anyhow::{anyhow, Result};
use num_bigint::BigUint;

const MODULUS: u32 = 20201227;
const BASE: u8 = 7;

pub fn part1(input: String) -> Result<BigUint> {
    let public_keys: Vec<BigUint> = input.lines()
        .map(|l| l.trim().parse::<BigUint>())
        .collect::<Result<_, _>>()?;

    let solved_key = find_private_key(&public_keys)
        .ok_or(anyhow!("cannot find private key"))?;

    Ok(
        public_keys.into_iter()
            .filter_map(|k| {
                if k == solved_key.public_key {
                    None
                } else {
                    Some(solved_key.encrypt(k))
                }
            }).next().unwrap()
    )
}

pub fn part2(_input: String) -> Result<BigUint> {
    Ok(0u8.into())
}

fn find_private_key(public_keys: &[BigUint]) -> Option<CryptoKey> {
    let mut exponent: BigUint = 1u8.into();
    let mut public_key: BigUint = BASE.into();

    while exponent < MODULUS.into() {
        if public_keys.contains(&public_key) {
            return Some(CryptoKey {
                public_key,
                private_key: exponent,
            });
        }

        exponent += 1u8;
        public_key = (public_key * &BASE) % &MODULUS;
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CryptoKey {
    public_key: BigUint,
    private_key: BigUint,
}

impl CryptoKey {
    fn encrypt(&self, val: BigUint) -> BigUint {
        val.modpow(&self.private_key, &(MODULUS.into()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "5764801\n17807724";

    #[test]
    fn part1_example() {
        assert_eq!(BigUint::from(14897079u32), part1(EXAMPLE.to_string()).unwrap());
    }
}

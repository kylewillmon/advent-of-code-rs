use anyhow::Result;
use aoclib::strtools;
use num_bigint::BigUint;
use num_integer::Integer;

pub fn part1(input: String) -> Result<u32> {
    let (time, buses) = strtools::split_once(input.as_str(), "\n");
    let time = time.parse::<u32>()?;
    let buses: Vec<u32> = buses.trim()
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<u32>())
        .collect::<Result<_, _>>()?;

    let (id, wait) = buses
        .into_iter()
        .map(|id| (id, wait_time(time, id)))
        .min_by_key(|&(_, wait)| wait)
        .unwrap();

    Ok(id * wait)
}

pub fn part2(input: String) -> Result<BigUint> {
    let (_, buses) = strtools::split_once(input.as_str(), "\n");
    let buses: Vec<(BigUint, BigUint)> = buses.trim()
        .split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, s)| s.parse::<usize>().map(|num| (i.into(), num.into())))
        .collect::<Result<_, _>>()?;

    let (mut timestamp, mut modulus) = buses.first().unwrap_or(&(0u8.into(), 0u8.into())).clone();

    for (minute, bus_id) in buses.into_iter().skip(1) {
        // This math only works if the Bus IDs are all coprime....
        assert_eq!(BigUint::from(1u8), modulus.clone().gcd(&bus_id));

        let minute = &bus_id - (&minute % &bus_id);
        while &timestamp % &bus_id != minute {
            timestamp += &modulus;
        }
        modulus *= bus_id;
    }
    Ok(timestamp)
}

fn wait_time(time: u32, bus: u32) -> u32 {
    let rem = time % bus;
    if rem == 0 {
        0
    } else {
        bus - rem
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "939\n7,13,x,x,59,x,31,19";

    #[test]
    fn part1_example() {
        assert_eq!(295, part1(EXAMPLE.to_string()).unwrap());
    }

    #[test]
    fn part2_example() {
        assert_eq!(BigUint::from(1068781u32), part2(EXAMPLE.to_string()).unwrap());
    }
}
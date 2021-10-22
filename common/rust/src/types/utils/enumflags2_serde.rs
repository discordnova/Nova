use enumflags2::{BitFlag, BitFlags};
use serde::{Deserialize, Deserializer};


pub fn from_enumflag2_truncated<'de, D, T>(deserializer: D) -> Result<BitFlags<T>, D::Error>
where
    D: Deserializer<'de>,
    T: BitFlag,
    T::Numeric: Deserialize<'de> + Into<u64>,
{
    let val = T::Numeric::deserialize(deserializer)?;
    Ok(BitFlags::<T>::from_bits_truncate(val))
}

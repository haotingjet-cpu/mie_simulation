use crate::mie::PI;
use std::error::Error;

pub(crate) fn find_x(diameter: f64, wavelength: f64) -> Result<f64, Box<dyn Error>> {
    let x = PI * diameter / wavelength;
    if x < 0.0 {
        return Err(From::from("diameter 或 wavelength 不能為負數"));
    }
    if x == 0.0 {
        return Err(From::from("diameter 不能等於 0"));
    }
    return Ok(x);
}

pub(crate) fn arange<T>(start: T, end: T) -> Result<Vec<f64>, Box<dyn Error>>
where
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: Error + 'static,
{
    Ok(arange_iter(start, end)?.collect())
}

pub(crate) fn arange_iter<T>(
    start: T,
    end: T,
) -> Result<impl DoubleEndedIterator<Item = f64> + Clone, Box<dyn Error>>
where
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: Error + 'static,
{
    let start_idx: usize = start.try_into()?;
    let end_idx: usize = end.try_into()?;

    Ok((start_idx..end_idx).map(|x| x as f64))
}

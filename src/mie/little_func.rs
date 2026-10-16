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

pub(crate) fn arange<T>(start: T, end: T, step: Option<f64>) -> Result<Vec<f64>, Box<dyn Error>>
where
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: Error + 'static,
{
    Ok(arange_iter(start, end, step)?.collect())
}

pub(crate) fn arange_iter<T>(
    start: T,
    end: T,
    step: Option<f64>,
) -> Result<impl DoubleEndedIterator<Item = f64> + Clone, Box<dyn Error>>
where
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: Error + 'static,
{
    let step = step.unwrap_or_else(|| 1.0f64);
    let start_idx: usize = start.try_into()?;
    let end_idx: usize = end.try_into()?;

    let steps_count = ((end_idx - start_idx) as f64 * step.recip()).round() as usize + 1;

    let iterable = (0..steps_count).map(move |x| (start_idx as f64) + (x as f64) * step);

    Ok(iterable)
}

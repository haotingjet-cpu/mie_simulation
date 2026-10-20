use hdf5::File;
pub fn storage_hdf5(
    ballistic: &[f64],
    scattered: &[f64],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("latest_result.hdf5")?;

    file.new_dataset_builder()
        .with_data(ballistic)
        .create("ballistic")?;

    file.new_dataset_builder()
        .with_data(scattered)
        .create("scattered")?;

    Err("test".into())
}

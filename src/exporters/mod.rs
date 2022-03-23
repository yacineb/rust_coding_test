use std::error::Error;

pub fn export_as_csv<W, T>(w: W, data: impl Iterator<Item = T>) -> Result<(), Box<dyn Error>>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    let mut wtr = csv::Writer::from_writer(w);
    for record in data {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

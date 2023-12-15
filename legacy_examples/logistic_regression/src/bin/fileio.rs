extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{Array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Our 2x3 test array
    let array = Array::from(vec![1, 2, 3, 4, 5, 6]).into_shape((6, 1)).unwrap();

    // Write the array into the file.
    {
        let file = File::create("test.csv")?;
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
        writer.serialize_array2(&array)?;
    }

    // Read an array back from the file
    let file = File::open("test.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<u64> = reader.deserialize_array2((6, 1))?;

    // Ensure that we got the original array back
    assert_eq!(array_read, array);
    Ok(())
}
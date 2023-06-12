use parquet::file::reader::{FileReader, SerializedFileReader};
use std::path::Path;
use std::fs::File;


fn load(path: &Path) {
   

    if let Ok(file) = File::open(&path) {

        let reader: SerializedFileReader<File> = SerializedFileReader::new(file).unwrap();

        let parquet_metadata: &parquet::file::metadata::ParquetMetaData = reader.metadata();
        assert_eq!(parquet_metadata.num_row_groups(), 1);

        let row_group_reader = reader.get_row_group(0).unwrap();
        assert_eq!(row_group_reader.num_columns(), 1);
    }
}

fn main() {

    let path: &Path = Path::new("/Users/arl/Dropbox/Code/py3/ctc-tools/test.tracks/nodes.parquet");

    load(path);
    println!("Hello, world!");
}

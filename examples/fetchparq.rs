use duckdb::arrow::record_batch::RecordBatch;
use duckdb::arrow::util::pretty::print_batches;
use duckdb::{Connection, Result};

#[derive(Debug)]
struct Person {
    fio: String,
    username: String,
    email: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    let mut stmt = conn.prepare(
        r"SELECT *
          FROM read_parquet('http://storage-bucket.web.garage.tld:3902/users_zstd.parquet');
        ",
    )?;

    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            fio: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("{:?}", person.unwrap());
    }

    // query table by arrow
    // let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
    // let _ = print_batches(&rbs);
    Ok(())
}

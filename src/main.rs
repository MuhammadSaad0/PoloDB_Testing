use gnuplot::{AxesCommon, Caption, Color, Figure, LineWidth};
use polodb_core::bson::{doc, Bson, Document};
use polodb_core::Database;
use polodb_core::{try_unwrap_document, Collection};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

fn add_books(times: i64, collection: Collection<Book>) {
    for i in 0..times {
        let _ = collection.insert_one(Book {
            title: i.to_string(),
            author: i.to_string(),
        });
    }
}

fn add_many_books(times: i64, collection: Collection<Book>) {
    let mut docs: Vec<Book> = Vec::new();
    for i in 0..times {
        docs.push(Book {
            title: i.to_string(),
            author: i.to_string(),
        })
    }
    collection.insert_many(docs);
}

fn main() {
    let mut durations = Vec::new();
    let db = Database::open_file("test-polo.db").unwrap();

    let num_runs = 1; // Number of runs to calculate the average

    for times in [1000000].iter() {
        let mut total_duration = Duration::new(0, 0);

        for _ in 0..num_runs {
            let start = Instant::now();
            let collection = db.collection("books");
            add_many_books(*times, collection);
            let duration = start.elapsed();
            total_duration += duration;
        }

        let average_duration = total_duration / num_runs as u32;
        durations.push(average_duration);

        println!(
            "Average Time Taken for {} times ({} runs): {:?}",
            times, num_runs, average_duration
        );
    }

    let x_values: Vec<f64> = [1000, 5000, 10000, 50000, 100000]
        .iter()
        .map(|&x| x as f64)
        .collect();
    let y_values: Vec<f64> = durations.iter().map(|d| d.as_millis() as f64).collect();

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Inserting a struct with two fields", &[])
        .set_x_label("Number of Inserts", &[])
        .set_y_label("Average Time (ms)", &[])
        .lines(&x_values, &y_values, &[Caption("Average Time")]);

    fg.show().unwrap();
}

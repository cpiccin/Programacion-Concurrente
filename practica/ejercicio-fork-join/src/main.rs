use csv::Reader;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rayon::ThreadPoolBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_dir, File};
use std::path::PathBuf;
use rayon::prelude::*;
use std::time::Instant;


const DATASET_DIR: &str = "src/no2_data/processed";

type QualityCounts = HashMap<String, u64>;

#[derive(Debug, Deserialize)]
struct Registro {
    no2_quality: String,
}

fn run_once(cant_threads: u32) -> Result<QualityCounts, Box<dyn Error>> {
    // Esto solo debería llamarse UNA vez en todo el programa
    ThreadPoolBuilder::new()
        .num_threads(cant_threads as usize)
        .build_global()
        .ok();

    let resultado = read_dir(DATASET_DIR)?
        .flatten()
        .map(|d| d.path())
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .flat_map(|path| {
            let file = File::open(path).unwrap();
            let reader = Reader::from_reader(file);
            reader
                .into_deserialize::<Registro>()
                .par_bridge()
        })
        .filter_map(|res| match res {
            Ok(registro) => Some(registro.no2_quality),
            Err(_) => None,
        })
        .fold(
            HashMap::new,
            |mut acc: QualityCounts, quality| {
                *acc.entry(quality).or_insert(0) += 1;
                acc
            },
        )
        .reduce(
            HashMap::new,
            |mut acc, map| {
                for (k, v) in map {
                    *acc.entry(k).or_insert(0) += v;
                }
                acc
            },
        );

    Ok(resultado)
}

fn main() -> Result<(), Box<dyn Error>> {

    // Recibo la cantidad de threads como argumento
    let args: Vec<String> = std::env::args().collect();
    let cant_threads = args.get(1).map(|s| s.parse::<u32>().unwrap_or(1)).unwrap_or(1);
    
    println!("Cantidad de hilos: {}", cant_threads);

    let start = Instant::now(); 

    run_once(cant_threads)?; 

    let duration = start.elapsed(); 

    println!("Tiempo total para {cant_threads} hilos: {:?}", duration);

    Ok(())
}
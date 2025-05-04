// This is the main entry point of the project. It loads the dataset, builds the graph, 
// computes centralities, performs delay analysis, and prints the results.

use std::error::Error;
use csv::Reader;
use crate::graph::Graph;
use crate::centrality::{closeness_centrality, degree_centrality};
use crate::delay_analysis::analyze_delays;

mod graph;
mod centrality;
mod delay_analysis;

#[derive(Debug)]
pub struct AirportFlight {
    carrier_name: String,
    airport_name: String,
    carrier_delay: f64,
    weather_delay: f64,
    nas_delay: f64,
    security_delay: f64,
    late_aircraft_delay: f64,
}

fn read_csv(file_path: &str) -> Result<Vec<AirportFlight>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut flights = Vec::new();

    for result in reader.records() {
        let record = result?;
        flights.push(AirportFlight {
            carrier_name: record.get(2).unwrap_or("").to_string(),
            airport_name: record.get(4).unwrap_or("").to_string(),
            carrier_delay: record.get(15).unwrap_or("0").parse().unwrap_or(0.0),
            weather_delay: record.get(16).unwrap_or("0").parse().unwrap_or(0.0),
            nas_delay: record.get(17).unwrap_or("0").parse().unwrap_or(0.0),
            security_delay: record.get(18).unwrap_or("0").parse().unwrap_or(0.0),
            late_aircraft_delay: record.get(19).unwrap_or("0").parse().unwrap_or(0.0),
        });
    }

    Ok(flights)
}

fn main() {
    let flights = read_csv("Airline_Delay_Cause.csv").expect("Failed to load data");

    println!("Loaded {} flight records.", flights.len());

    let mut graph = Graph::new();
    for flight in &flights {
        graph.add_edge(&flight.carrier_name, &flight.airport_name);
    }

    println!("Graph built with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    let closeness = closeness_centrality(&graph);
    let degree = degree_centrality(&graph);
    let delays = analyze_delays(&flights);

    println!("\nTop by closeness centrality:");
    print_top_10(&closeness);

    println!("\nTop by degree centrality:");
    print_top_10_usize(&degree);

    println!("\nTop 10 airports by delay category:");
    for (category, top_airports) in delays {
        println!("\n{}:", category);
        for (airport, value) in top_airports {
            println!("{}: {:.2}", airport, value);
        }
    }
}

fn print_top_10(map: &std::collections::HashMap<String, f64>) {
    let mut vec: Vec<_> = map.iter().collect();
    vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    for (k, v) in vec.iter().take(10) {
        println!("{}: {:.4}", k, v);
    }
}

fn print_top_10_usize(map: &std::collections::HashMap<String, usize>) {
    let mut vec: Vec<_> = map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    for (k, v) in vec.iter().take(10) {
        println!("{}: {}", k, v);
    }
}

// Analyzes delay categories and summarizes top airports for each delay type.

use std::collections::HashMap;
use crate::AirportFlight;

/// Analyzes delays and returns top airports per category.
/// Inputs: flight slice.
/// Output: HashMap of category to top airport list.
pub fn analyze_delays(flights: &[AirportFlight]) -> HashMap<String, Vec<(String, f64)>> {
    let mut categories = vec![
        ("carrier_delay", HashMap::new()),
        ("weather_delay", HashMap::new()),
        ("nas_delay", HashMap::new()),
        ("security_delay", HashMap::new()),
        ("late_aircraft_delay", HashMap::new()),
    ];

    // Sum delays per airport.
    for flight in flights {
        categories[0].1.entry(flight.airport_name.clone()).and_modify(|v| *v += flight.carrier_delay).or_insert(flight.carrier_delay);
        categories[1].1.entry(flight.airport_name.clone()).and_modify(|v| *v += flight.weather_delay).or_insert(flight.weather_delay);
        categories[2].1.entry(flight.airport_name.clone()).and_modify(|v| *v += flight.nas_delay).or_insert(flight.nas_delay);
        categories[3].1.entry(flight.airport_name.clone()).and_modify(|v| *v += flight.security_delay).or_insert(flight.security_delay);
        categories[4].1.entry(flight.airport_name.clone()).and_modify(|v| *v += flight.late_aircraft_delay).or_insert(flight.late_aircraft_delay);
    }

    let mut result = HashMap::new();
    for (name, map) in categories {
        let mut vec: Vec<_> = map.into_iter().collect();
        vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        result.insert(name.to_string(), vec.into_iter().take(10).collect());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analyze_delays() {
        let flights = vec![
            AirportFlight {
                carrier_name: "A".to_string(),
                airport_name: "X".to_string(),
                carrier_delay: 10.0,
                weather_delay: 5.0,
                nas_delay: 2.0,
                security_delay: 1.0,
                late_aircraft_delay: 3.0,
            },
            AirportFlight {
                carrier_name: "B".to_string(),
                airport_name: "X".to_string(),
                carrier_delay: 5.0,
                weather_delay: 2.0,
                nas_delay: 1.0,
                security_delay: 0.0,
                late_aircraft_delay: 2.0,
            },
        ];
        let delays = analyze_delays(&flights);
        assert!(delays["carrier_delay"].iter().any(|(name, _)| name == "X"));
    }
}

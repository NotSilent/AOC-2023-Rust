struct AlmanacRange {
    destination: u64,
    source: u64,
    range: u64,
}

impl AlmanacRange {
    fn get_destination(&self, source: u64) -> Option<u64> {
        if self.source <= source && source < self.source + self.range {
            let offset = source - self.source;

            return Some(self.destination + offset);
        }

        None
    }
}

struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl From<&str> for AlmanacMap {
    fn from(value: &str) -> Self {
        let mut almanac_ranges: Vec<AlmanacRange> = vec![];

        let mut split = value.split(":\n");
        if let Some(_description) = split.next() {
            if let Some(ranges) = split.next() {
                for line in ranges.lines() {
                    let mut values = line.split_whitespace();
                    if let Some(destination) = values.next() {
                        if let Some(source) = values.next() {
                            if let Some(range) = values.next() {
                                almanac_ranges.push(AlmanacRange {
                                    destination: destination.parse::<u64>().unwrap(),
                                    source: source.parse::<u64>().unwrap(),
                                    range: range.parse::<u64>().unwrap(),
                                });
                            }
                        }
                    }
                }
            }
        }

        AlmanacMap {
            ranges: almanac_ranges,
        }
    }
}

impl AlmanacMap {
    fn get_destination(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if let Some(destination) = range.get_destination(source) {
                return destination;
            }
        }

        source
    }
}

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut split = value.split("\n\n");

        let seeds_text = split.next().unwrap().split(':');
        let seeds: Vec<u64> = seeds_text.last().unwrap().split_whitespace().map(|seed| seed.parse::<u64>().unwrap()).collect();

        let seed_to_soil: AlmanacMap = split.next().unwrap().into();
        let soil_to_fertilizer: AlmanacMap = split.next().unwrap().into();
        let fertilizer_to_water: AlmanacMap = split.next().unwrap().into();
        let water_to_light: AlmanacMap = split.next().unwrap().into();
        let light_to_temperature: AlmanacMap = split.next().unwrap().into();
        let temperature_to_humidity: AlmanacMap = split.next().unwrap().into();
        let humidity_to_location: AlmanacMap = split.next().unwrap().into();

        Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

impl Almanac {
    fn get_seed_location_number(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get_destination(seed);
        let fertilizer = self.soil_to_fertilizer.get_destination(soil);
        let water = self.fertilizer_to_water.get_destination(fertilizer);
        let light = self.water_to_light.get_destination(water);
        let temperature = self.light_to_temperature.get_destination(light);
        let humidity = self.temperature_to_humidity.get_destination(temperature);

        self.humidity_to_location.get_destination(humidity)
    }

    fn get_lowest_location_number(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.get_seed_location_number(*seed))
            .min().unwrap()
    }

    fn get_lowest_location_number_from_ranges(&self) -> u64 {
        self.seeds.chunks(2)
            .map(|chunk| {
                let mut chunk_iter = chunk.iter();
                if let Some(initial_seed) = chunk_iter.next() {
                    if let Some(range) = chunk_iter.next() {
                        return (*initial_seed..*initial_seed + *range - 1)
                            .map(|seed| self.get_seed_location_number(seed))
                            .min().unwrap();
                    }
                }

                u64::MAX
            })
            .min()
            .unwrap()
    }
}

fn main() {
    let text = include_str!("puzzle_input.txt");

    let almanac: Almanac = text.into();

    println!("Puzzle 0: {}\nPuzzle 1: {}", almanac.get_lowest_location_number(), almanac.get_lowest_location_number_from_ranges(), )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let text = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let almanac: Almanac = text.into();

        assert_eq!(almanac.get_lowest_location_number(), 35);
        assert_eq!(almanac.get_lowest_location_number_from_ranges(), 46);
    }

    #[test]
    fn puzzle_0() {
        let text = include_str!("puzzle_input.txt");

        let almanac: Almanac = text.into();

        assert_eq!(almanac.get_lowest_location_number(), 579439039);
    }

    //#[test]
    fn puzzle_1() {
        let text = include_str!("puzzle_input.txt");

        let almanac: Almanac = text.into();

        assert_eq!(almanac.get_lowest_location_number_from_ranges(), 7873084);
    }
}
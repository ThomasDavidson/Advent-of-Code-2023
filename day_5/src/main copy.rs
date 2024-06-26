fn get_number_lists(lines: &str) -> Vec<Vec<i64>> {
    let mut maps: Vec<Vec<i64>> = vec![];

    for line in lines.lines() {
        let map: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        if map.len() > 0 {
            maps.push(map);
        }
    }

    maps
}

fn calculate_map_value(value: i64, maps: &Vec<Vec<i64>>) -> i64 {
    for map in maps {
        let lower_check = match map.get(1) {
            None => panic!("0 Should not be none"),
            Some(val) => val.clone(),
        };
        let upper_check = match map.get(2) {
            None => panic!("2 Should not be none"),
            Some(val) => lower_check + val - 1,
        };
        let offset = match map.get(0) {
            None => panic!("1 Should not be none"),
            Some(val) => val.clone(),
        };
        // println!("value: {}, upper: {} lower: {} offset: {}", value, upper_check, lower_check, offset);
        if value <= upper_check && value >= lower_check {
            return value - lower_check + offset;
        }
    }

    value
}

fn get_location_from_seed(
    seed: i64,
    seed_to_soil_maps: &Vec<Vec<i64>>,
    soil_to_fertilizer_maps: &Vec<Vec<i64>>,
    fertilizer_to_water_maps: &Vec<Vec<i64>>,
    water_to_light_maps: &Vec<Vec<i64>>,
    light_to_temperature_maps: &Vec<Vec<i64>>,
    temperature_to_humidity_maps: &Vec<Vec<i64>>,
    humidity_to_location_maps: &Vec<Vec<i64>>,
) -> i64 {
    let soil = calculate_map_value(seed, seed_to_soil_maps);
    let furt = calculate_map_value(soil, soil_to_fertilizer_maps);
    let water = calculate_map_value(furt, fertilizer_to_water_maps);
    let light = calculate_map_value(water, water_to_light_maps);
    let temp = calculate_map_value(light, light_to_temperature_maps);
    let humiditiy = calculate_map_value(temp, temperature_to_humidity_maps);
    calculate_map_value(humiditiy, humidity_to_location_maps)
}

fn main() {
    let input: &str = include_str!("../input.txt");

    let mut sections = input.split("\r\n\r\n");

    let seed_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };

    let seed_nums: Vec<i64> = seed_str
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    // println!("seed_nums:{:?}", seed_nums);

    let seed_to_soil_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };
    let seed_to_soil_maps = get_number_lists(seed_to_soil_str);

    // println!("seed_to_soil:{:?}", seed_to_soil_maps);

    let soil_to_fertilizer_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };
    let soil_to_fertilizer_maps = get_number_lists(soil_to_fertilizer_str);
    // println!("soil_to_fertilizer_maps:{:?}", soil_to_fertilizer_maps);

    let fertilizer_to_water_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };
    let fertilizer_to_water_maps = get_number_lists(fertilizer_to_water_str);
    // println!("fertilizer_to_water_maps:{:?}", fertilizer_to_water_maps);

    let water_to_light_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };
    let water_to_light_maps = get_number_lists(water_to_light_str);
    // println!("water_to_light_maps:{:?}", water_to_light_maps);

    let light_to_temperature_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };
    let light_to_temperature_maps = get_number_lists(light_to_temperature_str);
    // println!("light_to_temperature_maps:{:?}", light_to_temperature_maps);

    let temperature_to_humidity_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };
    let temperature_to_humidity_maps = get_number_lists(temperature_to_humidity_str);
    // println!(
    //     "temperature_to_humidity_maps:{:?}",
    //     temperature_to_humidity_maps
    // );

    let humidity_to_location_str = match sections.next() {
        None => panic!("Should not be none"),
        Some(i) => i,
    };
    let humidity_to_location_maps = get_number_lists(humidity_to_location_str);
    // println!("humidity_to_location_maps:{:?}", humidity_to_location_maps);

    let mut day_1_answer: Vec<i64> = vec![];
    for seed in seed_nums.clone() {
        let location = get_location_from_seed(
            seed,
            &seed_to_soil_maps,
            &soil_to_fertilizer_maps,
            &fertilizer_to_water_maps,
            &water_to_light_maps,
            &light_to_temperature_maps,
            &temperature_to_humidity_maps,
            &humidity_to_location_maps,
        );

        day_1_answer.push(location);
    }
    println!("day_1_answer: {:?}", day_1_answer.iter().min());

    let day_2_seed_chunk = seed_nums.chunks(2);
    let day_2_seed_chunk_len = day_2_seed_chunk.len();

    let mut day_2_answers: Vec<i64> = vec![];

    for (i, day_2_seed_pair) in day_2_seed_chunk.into_iter().enumerate() {
        println!("{} out of {}", i, day_2_seed_chunk_len);

        let seed_start = match day_2_seed_pair.get(0) {
            None => panic!("0 Should not be none"),
            Some(val) => val.clone(),
        };
        let length = match day_2_seed_pair.get(1) {
            None => panic!("1 Should not be none"),
            Some(val) => val.clone(),
        };

        let seed_range = seed_start..seed_start + length;

        for day_2_seed in seed_range {
            let location = get_location_from_seed(
                day_2_seed.clone(),
                &seed_to_soil_maps,
                &soil_to_fertilizer_maps,
                &fertilizer_to_water_maps,
                &water_to_light_maps,
                &light_to_temperature_maps,
                &temperature_to_humidity_maps,
                &humidity_to_location_maps,
            );
            day_2_answers.push(location);
        }
    }

    println!("day_2_answers: {:?}", day_2_answers);
    println!("day_2_answer: {:?}", day_2_answers.iter().min());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_map() {
        let seed_to_soil: Vec<Vec<i64>> = [[50, 98, 2].to_vec(), [52, 50, 48].to_vec()].to_vec();
        let result = calculate_map_value(98, &seed_to_soil);
        assert_eq!(result, 50);
        let result = calculate_map_value(79, &seed_to_soil);
        assert_eq!(result, 81);
        let result = calculate_map_value(14, &seed_to_soil);
        assert_eq!(result, 14);
        let result = calculate_map_value(55, &seed_to_soil);
        assert_eq!(result, 57);
        let result = calculate_map_value(13, &seed_to_soil);
        assert_eq!(result, 13);
    }
}

use std::collections::HashMap;

fn get_number_lists(lines: &str) -> Vec<[i64; 3]> {
    let mut maps: Vec<Vec<i64>> = vec![];
    let mut array_maps: Vec<[i64; 3]> = Vec::new();
    let mut array_map: [i64; 3];

    for line in lines.lines() {
        let map: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        if map.len() > 0 {
            maps.push(map);
        }
    }

    // println!("Maps");
    for map in &maps {
        array_map = map.clone().try_into().unwrap();
        array_maps.push(array_map)
    }
    // println!("{:?}", array_maps);

    array_maps
}

fn calculate_map_value(value: i64, maps: &Vec<[i64; 3]>) -> i64 {
    for map in maps {
        let lower_check = map[1];
        let upper_check = lower_check + map[2] - 1;
        let offset = map[0];
        // println!("value: {}, upper: {} lower: {} offset: {}", value, upper_check, lower_check, offset);
        if value <= upper_check && value >= lower_check {
            return value - lower_check + offset;
        }
    }
    value
}

fn get_location_from_seed(seed: i64, maps: &Vec<Vec<[i64; 3]>>) -> i64 {
    let mut res = seed;
    for map in maps {
        print!(" {}", res);
        res = calculate_map_value(res, map);
    }
    println!(" {}", res);
    res
}

fn get_location_from_hash_maps(seed: i64, hash_maps: Vec<HashMap<i64, i64>>) -> i64 {
    let mut res = seed;
    for hash_map in hash_maps {
        res = match hash_map.get(&res) {
            Some(val) => val.to_owned(),
            None => res,
        };

        print!(" {}", res);
    }
    println!(" {}", res);
    res
}

fn create_hash_map_from_map(maps: &Vec<[i64; 3]>) -> HashMap<i64, i64> {
    let mut hash_map: HashMap<i64, i64> = HashMap::new();

    for map in maps {
        let destination = map[0];
        let source = map[1];
        let range = map[2];

        for i in 0..range {
            let k = source + i;
            let v = destination + i;
            hash_map.insert(k, v);
        }
    }

    hash_map
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

    let mut seed_map_maps: Vec<Vec<[i64; 3]>> = Vec::new();
    let mut seed_map_hash_map: Vec<HashMap<i64, i64>> = Vec::new();

    for (i, section) in sections.into_iter().enumerate() {
        let maps: Vec<[i64; 3]> = get_number_lists(section);
        // println!("{:?}", maps);
        seed_map_maps.push(maps.clone());
        seed_map_hash_map.push(create_hash_map_from_map(&maps));
    }

    let mut day_1_answer: Vec<i64> = vec![];
    for seed in seed_nums.clone() {
        let location = get_location_from_hash_maps(seed, seed_map_hash_map.clone());

        println!("");
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
            let location = get_location_from_hash_maps(day_2_seed, seed_map_hash_map.clone());
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
        let seed_to_soil: Vec<[i64; 3]> = [[50, 98, 2], [52, 50, 48]].to_vec();
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

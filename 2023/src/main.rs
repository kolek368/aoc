use core::str;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

#[allow(dead_code)]
fn aoc_print_vec(input: &Vec<i32>) {
    // do not delete this function, to no forget such silly behavior :D
    for v in input.into_iter() {
        print!("{} ", v);
    }
    println!("");
}

fn aoc_gcm(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    println!("AOC_LIB Checking: {} {}", a, b);
    if a > b {
        return aoc_gcm(b, a%b);
    } else {
        return aoc_gcm(a, b%a);
    }
}

fn aoc_lcm(a: u64, b: u64) -> u64 {
    return a*b / aoc_gcm(a, b);
}

fn day_1_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 1 part 1");
    let mut result = 0;

    for (_,line) in input_lines.into_iter().enumerate() {
        //println!("{}", line);
        let mut first_num = -1;
        let mut last_num = -1;
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                last_num = c as i32 - '0' as i32;
                if first_num < 0 {
                    first_num = last_num;
                }
            }
        }
        //println!("First: {} Last: {}", first_num, last_num);
        result += 10*first_num + last_num;
    }

    println!("Final result: {}", result);
}

fn day_1_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 1 part 2");
    let mut result = 0;

    let string_numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (_,line) in input_lines.into_iter().enumerate() {
        //println!("{}", line);
        let mut first_num_idx = -1;
        let mut last_num_idx = -1;
        let mut first_num = 0;
        let mut last_num = 0;
        for idx in 0..line.chars().count() {
            let c = line.chars().nth(idx).unwrap();
            if c >= '0' && c <= '9' {
                last_num_idx = idx as i32;
                last_num = c as i32 - '0' as i32;
                if first_num_idx < 0 {
                    first_num_idx = idx as i32;
                    first_num = last_num;
                }
            }
        }

        for number in string_numbers.keys() {
            let tmp = line.find(number);
            if tmp.is_none() {
                continue;
            }
            let tmp = tmp.unwrap() as i32;
            if first_num_idx < 0 || tmp < first_num_idx {
                first_num_idx = tmp;
                first_num = string_numbers[number];
            }
            let tmp = line.rfind(number).unwrap() as i32;
            if last_num_idx < 0 || tmp > last_num_idx {
                last_num_idx = tmp;
                last_num = string_numbers[number];
            }
        }

        //println!("First: {} Last: {}", first_num, last_num);
        result += 10*first_num + last_num;
    }

    println!("Final result: {}", result);
}

fn day_2_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 2 part 1");
    let balls_limits = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut result = 0;

    for (_,line) in input_lines.into_iter().enumerate() {
        let mut idx = 5;
        let mut game_number = 0;
        for val in line.chars().skip(idx) {
            if val == ':' {
                break;
            }
            game_number = game_number*10 + (val as i32 - '0' as i32) as i32;
            idx += 1;
        }
        //println!("Parsing game number {}", game_number);
        //println!("\tInput: {}", line);
        let mut ball_cnt = 0;
        let mut color_idx = 0;
        let mut valid = true;
        let game = &line[idx+1..line.chars().count()];
        for (idx, val) in game.chars().enumerate() {
            //println!("Idx: {} val: {}", idx, val);
            if val == ' ' {
                continue;
            }
            if val == ';' || val == ',' || idx == game.chars().count()-1 {
                let ball_color = if idx == game.chars().count()-1 {
                    &game[color_idx..idx+1]
                } else {
                    &game[color_idx..idx]
                };
                //println!("Balls color: {} count: {}", &ball_color, ball_cnt);
                if ball_cnt > *balls_limits.get(ball_color).unwrap() {
                    valid = false;
                    break;
                }
                ball_cnt = 0;
                color_idx = 0;
            } else if val >= '0' && val <= '9' {
                ball_cnt = ball_cnt * 10 + (val as i32 - '0' as i32);
            } else if color_idx == 0 {
                color_idx = idx;
            }
        }
        if valid {
            result += game_number;
        }
    }

    println!("Final result: {}", result);
}

fn day_2_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 2 part 2");
    let mut result = 0;

    for (_,line) in input_lines.into_iter().enumerate() {
        let mut idx = 5;
        let mut game_number = 0;
        for val in line.chars().skip(idx) {
            if val == ':' {
                break;
            }
            game_number = game_number*10 + (val as i32 - '0' as i32) as i32;
            idx += 1;
        }
        //println!("Parsing game number {}", game_number);
        //println!("\tInput: {}", line);
        let mut ball_cnt = 0;
        let mut color_idx = 0;
        let mut balls_limits = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        let game = &line[idx+1..line.chars().count()];
        for (idx, val) in game.chars().enumerate() {
            //println!("Idx: {} val: {}", idx, val);
            if val == ' ' {
                continue;
            }
            if val == ';' || val == ',' || idx == game.chars().count()-1 {
                let ball_color = if idx == game.chars().count()-1 {
                    &game[color_idx..idx+1]
                } else {
                    &game[color_idx..idx]
                };
                if ball_cnt > *balls_limits.get(ball_color).unwrap() {
                    *balls_limits.get_mut(ball_color).unwrap() = ball_cnt;
                }
                ball_cnt = 0;
                color_idx = 0;
            } else if val >= '0' && val <= '9' {
                ball_cnt = ball_cnt * 10 + (val as i32 - '0' as i32);
            } else if color_idx == 0 {
                color_idx = idx;
            }
        }
        result += *balls_limits.get("red").unwrap() *  *balls_limits.get("green").unwrap() *  *balls_limits.get("blue").unwrap();
    }

    println!("Final result: {}", result);
}

fn d3p1_check(input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize) -> bool {
    let mut result = false;
    let check_nbrs = |input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize| -> bool {
        for idx in (col_start as i32 - 1)..(col_stop as i32 + 1) {
            if idx < 0 {
                continue;
            }
            if idx as usize >= input_lines[0].len() {
                continue;
            }
            let c = input_lines[row].chars().nth(idx as usize).unwrap();
            if c != '.' && (c <= '0' || c >= '9') {
                return true;
            }
        }
        return false;
    };
    if row > 0 {
        result |= check_nbrs(input_lines, row-1, col_start, col_stop);
    }

    if col_start > 0 {
        let c = input_lines[row].chars().nth(col_start - 1).unwrap();
        result |= c != '.' && (c <= '0' || c >= '9');
    }

    if col_stop < input_lines[0].chars().count() {
        let c = input_lines[row].chars().nth(col_stop).unwrap();
        result |= c != '.' && (c <= '0' || c >= '9');
    }

    if row+1 < input_lines.len() {
        result |= check_nbrs(input_lines, row+1, col_start, col_stop);
    }
    return result;
}

fn day_3_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 3 part 1");
    let mut result = 0;
    for (idx,line) in input_lines.into_iter().enumerate() {
        //println!("Parsing line {}: {}", idx, line);
        let mut val = 0;
        let mut start_idx = 0;
        for (jdx, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' {
                if val == 0 {
                    start_idx = jdx;
                }
                val = val * 10 + (c as i32 - '0' as i32);
            }

            let is_last_char = jdx == line.chars().count()-1;

            if c < '0' || c > '9' || is_last_char {
                let offset = if is_last_char {
                    1
                } else {
                    0
                };
                // process new number additional chack for numbers on the right edge
                if d3p1_check(input_lines, idx, start_idx, jdx + offset) {
                    result += val;
                }
                //else {
                //    println!("Invalid value: {}", val);
                //}
                val = 0;
            }
        }
    }
    println!("Final result: {}", result);
}

fn d3p2_check(input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    let check_nbrs = |input_lines: &Vec<String>, row: usize, col_start: usize, col_stop: usize, result: &mut Vec<(usize, usize)>| {
        for idx in (col_start as i32 - 1)..(col_stop as i32 + 1) {
            if idx < 0 {
                continue;
            }
            if idx as usize >= input_lines[0].len() {
                continue;
            }
            let c = input_lines[row].chars().nth(idx as usize).unwrap();
            if c == '*' {
                result.push((row, idx as usize))
            }
        }
    };
    if row > 0 {
        check_nbrs(input_lines, row-1, col_start, col_stop, &mut result);
    }

    if col_start > 0 {
        let c = input_lines[row].chars().nth(col_start - 1).unwrap();
        if c == '*' {
            result.push((row, col_start - 1));
        }
    }

    if col_stop < input_lines[0].chars().count() {
        let c = input_lines[row].chars().nth(col_stop).unwrap();
        if c == '*' {
            result.push((row, col_stop));
        }
    }

    if row+1 < input_lines.len() {
        check_nbrs(input_lines, row+1, col_start, col_stop, &mut result);
    }
    return result;
}

fn d3p2_update_lut(lut: &mut HashMap<(usize, usize), Vec<i32>>, positions: &Vec<(usize, usize)>, val: i32) {
    for position in positions.into_iter() {
        lut.entry(*position).and_modify(|nbrs| (*nbrs).push(val)).or_insert(vec![val]);
    }
}

fn day_3_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 3 part 2");
    let mut result = 0;
    let mut gears_lut: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for (idx,line) in input_lines.into_iter().enumerate() {
        //println!("Parsing line {}: {}", idx, line);
        let mut val = 0;
        let mut start_idx = 0;
        for (jdx, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' {
                if val == 0 {
                    start_idx = jdx;
                }
                val = val * 10 + (c as i32 - '0' as i32);
            }

            let is_last_char = jdx == line.chars().count()-1;

            if (c < '0' || c > '9' || is_last_char) && val != 0 {
                // process new number additional check for numbers on the right edge
                let offset = if is_last_char {
                    1
                } else {
                    0
                };
                d3p2_update_lut(&mut gears_lut, &d3p2_check(input_lines, idx, start_idx, jdx + offset), val);
                val = 0;
            }
        }
    }
    for gear in gears_lut.into_iter() {
        let mut tmp = 1;
        //print!("Gear: {},{} ", gear.0.0, gear.0.1);
        if gear.1.len() != 2 {
            continue;
        }
        for v in gear.1 {
            //print!("{},", v);
            tmp = tmp * v;
        }
        result += tmp;
    }
    println!("Final result: {}", result);
}

fn day_4_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 4 part 1");
    let mut result = 0;
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        let mut jdx = 5;
        let mut card_num = 0;
        let mut winning: Vec<i32> = vec![];
        let mut chosen: Vec<i32> = vec![];
        while jdx < line.chars().count() {
            // Get card number
            let c = line.chars().nth(jdx).unwrap();
            jdx += 1;
            if c == ':' {
                break;
            }
            if c >= '0' && c <= '9' {
                card_num = card_num * 10 + (c as i32 - '0' as i32);
            }
        }

        let mut val = 0;
        while jdx < line.chars().count() {
            // Get winning numbers
            let c = line.chars().nth(jdx).unwrap();
            if c == '|' {
                break;
            }

            if c == ' ' && val > 0 {
                winning.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        val = 0;
        while jdx < line.chars().count() {
            // Get chosen numbers
            let c = line.chars().nth(jdx).unwrap();

            if c == ' ' && val > 0 {
                chosen.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        if val > 0 {
            chosen.push(val);
        }

        val = 0;
        let winning_set: HashSet<i32> = HashSet::from_iter(winning.into_iter());
        for v in chosen {
            if winning_set.contains(&v) {
                val += 1;
            }
        }
        if val > 0 {
            result += 1 << (val-1);
        }
    }
    println!("Final result: {}", result);
}

fn day_4_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 4 part 2");
    let mut result = 0;
    let mut cards_cnt: HashMap<i32, i32> = HashMap::from([]);
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        let mut jdx = 5;
        let mut card_num = 0;
        let mut winning: Vec<i32> = vec![];
        let mut chosen: Vec<i32> = vec![];
        while jdx < line.chars().count() {
            // Get card number
            let c = line.chars().nth(jdx).unwrap();
            jdx += 1;
            if c == ':' {
                break;
            }
            if c >= '0' && c <= '9' {
                card_num = card_num * 10 + (c as i32 - '0' as i32);
            }
        }

        let mut val = 0;
        while jdx < line.chars().count() {
            // Get winning numbers
            let c = line.chars().nth(jdx).unwrap();
            if c == '|' {
                break;
            }

            if c == ' ' && val > 0 {
                winning.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        val = 0;
        while jdx < line.chars().count() {
            // Get chosen numbers
            let c = line.chars().nth(jdx).unwrap();

            if c == ' ' && val > 0 {
                chosen.push(val);
                val = 0;
            } else if c >= '0' && c <= '9' {
                val = val * 10 + (c as i32 - '0' as i32);
            }
            jdx += 1;
        }

        if val > 0 {
            chosen.push(val);
        }

        val = 0;
        let winning_set: HashSet<i32> = HashSet::from_iter(winning.into_iter());
        for v in chosen {
            if winning_set.contains(&v) {
                val += 1;
            }
        }

        cards_cnt.entry(card_num).or_insert(1);
        for idx in 1..val + 1 {
            let multiplier = cards_cnt[&card_num];
            if card_num + idx <= input_lines.len() as i32 {
                println!("Parsing card: {} multiplier: {} idx: {}", card_num, multiplier, idx);
                *cards_cnt.entry(card_num+idx).or_insert(1) += multiplier;
            } else {
                println!("Index out of bounds: {}", card_num + idx);
            }
        }
    }
    for card_cnt in cards_cnt {
        println!("Card {}:{}", card_cnt.0, card_cnt.1);
        result += card_cnt.1;
    }
    println!("Final result: {}", result);
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum D5P1State {
    Idle,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Clone)]
struct D5P1Range {
    src: u64,
    dst: u64,
    len: u64,
}

fn d5p1_parse_map(line: &String, ranges: &mut Vec<D5P1Range>) {
    let vals: Vec<&str> = line.split(" ").collect();
    assert_eq!(vals.len(), 3);
    let dst = vals[0].parse::<u64>().unwrap();
    let src = vals[1].parse::<u64>().unwrap();
    let len = vals[2].parse::<u64>().unwrap();
    ranges.push(D5P1Range{src: src, dst: dst, len: len});

}

fn d5p1_get_dst(src: u64, ranges: &Vec<D5P1Range>) -> u64 {
    for range in ranges {
        if src >= range.src && src < (range.src + range.len) {
            return range.dst + (src - range.src);
        }
    }
    return src;
}

fn day_5_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 5 part 2");
    let mut result: u64 = u64::MAX;
    let mut state: D5P1State = D5P1State::Idle;
    let mut states_lut: HashMap<D5P1State, Vec<D5P1Range>> = HashMap::new();
    let mut tmp_ranges: Vec<D5P1Range> = vec![];
    let mut seeds: Vec<&str> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        if line.is_empty() {
            println!("Skipping empty line.");
            if !matches!(state, D5P1State::Idle) {
                println!("Current state: {:?}", state);
                states_lut.insert(state, tmp_ranges.clone());
                tmp_ranges.clear();
            }
            state = D5P1State::Idle;
            continue;
        }

        if line.starts_with("seeds: ") {
            seeds = line.split_at(7).1.split(' ').collect();
            println!("Seeds: {:?}", seeds);
        }
        else if line.starts_with("seed-to-soil map:") {
            state = D5P1State::SeedToSoil;
        }
        else if line.starts_with("soil-to-fertilizer map:") {
            state = D5P1State::SoilToFertilizer;
        }
        else if line.starts_with("fertilizer-to-water map:") {
            state = D5P1State::FertilizerToWater;
        }
        else if line.starts_with("water-to-light map:") {
            state = D5P1State::WaterToLight;
        }
        else if line.starts_with("light-to-temperature map:") {
            state = D5P1State::LightToTemperature;
        }
        else if line.starts_with("temperature-to-humidity map:") {
            state = D5P1State::TemperatureToHumidity;
        }
        else if line.starts_with("humidity-to-location map:") {
            state = D5P1State::HumidityToLocation;
        } else {
            println!("Parsing map entry: {}", line);
            d5p1_parse_map(&line, &mut tmp_ranges);
        }
    }

    for seed in seeds {
        let mut tmp_src: u64 = seed.parse().unwrap();
        println!("Processing seed: {}", tmp_src);
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::SeedToSoil).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::SoilToFertilizer).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::FertilizerToWater).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::WaterToLight).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::LightToTemperature).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::TemperatureToHumidity).unwrap());
        tmp_src = d5p1_get_dst(tmp_src, &states_lut.get(&D5P1State::HumidityToLocation).unwrap());
        if result > tmp_src {
            result = tmp_src;
        }
    }
    println!("Final result: {}", result);
}

#[derive(Debug, Clone)]
struct D5P2Range {
    src: u64,
    len: u64,
}

fn d5p2_get_dst<'a>(seed: &D5P2Range, ranges: &'a Vec<D5P1Range>) -> Option<&'a D5P1Range> {
    for range in ranges {
        if seed.src >= range.src && seed.src < (range.src + range.len) {
            // Start of range fits
            return Some(&range);
        } else if (seed.src + seed.len) >= range.src && (seed.src + seed.len) < (range.src + range.len) {
            // End of range fits
            return Some(&range);
        }
    }
    // Seed did not fit any range so return 1:1 mapping
    None
}

fn d5p2_process(seeds: &mut Vec<D5P2Range>, lut: &Vec<D5P1Range>) -> Vec<D5P2Range> {
    let mut result: Vec<D5P2Range> = vec![];
    while !seeds.is_empty() {
        let seed = seeds.pop().unwrap();
        println!("Parsing seeds at: {} range: {}", seed.src, seed.len);
        let tmp_seed = d5p2_get_dst(&seed, lut);
        if !tmp_seed.is_some() {
            // Empty result means 1:1 mapping
            result.push(seed);
        } else {
            let tmp_range = tmp_seed.unwrap();
            if seed.src >= tmp_range.src && seed.src + seed.len < tmp_range.src + tmp_range.len {
                // All current range fits in new mapping
                println!("Range fits");
                result.push(D5P2Range { src: tmp_range.dst + seed.src - tmp_range.src, len: seed.len });
            } else if seed.src >= tmp_range.src && seed.src + seed.len >= tmp_range.src + tmp_range.len {
                // Begining of current range fits in new mapping
                println!("Left side of the range fits");
                let offset = seed.src - tmp_range.src;
                let fit_len = tmp_range.len - offset;
                result.push(D5P2Range { src: tmp_range.dst + offset, len: fit_len });
                seeds.push(D5P2Range { src: tmp_range.src + tmp_range.len, len: seed.len - fit_len });
            } else {
                // End of current range fits in new mapping
                println!("Right side of the range fits[src {} {}| range {} {}]", seed.src, seed.len, tmp_range.src, tmp_range.len);
                // Add one because first element from the tmp_range is also parsed already
                let fit_len = seed.len - (tmp_range.src - seed.src) + 1;
                result.push(D5P2Range { src: tmp_range.dst, len: fit_len });
                seeds.push(D5P2Range { src: seed.src, len: seed.len - fit_len });
            }
        }
        //result.push(tmp_seed);
    }
    return result;
}

fn day_5_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 5 part 2");
    let mut result: u64 = u64::MAX;
    let mut state: D5P1State = D5P1State::Idle;
    let mut states_lut: HashMap<D5P1State, Vec<D5P1Range>> = HashMap::new();
    let mut tmp_ranges: Vec<D5P1Range> = vec![];
    let mut seeds: Vec<&str> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        if line.is_empty() {
            //println!("Skipping empty line.");
            if !matches!(state, D5P1State::Idle) {
                //println!("Current state: {:?}", state);
                states_lut.insert(state, tmp_ranges.clone());
                tmp_ranges.clear();
            }
            state = D5P1State::Idle;
            continue;
        }

        if line.starts_with("seeds: ") {
            seeds = line.split_at(7).1.split(' ').collect();
            //println!("Seeds: {:?}", seeds);
        }
        else if line.starts_with("seed-to-soil map:") {
            state = D5P1State::SeedToSoil;
        }
        else if line.starts_with("soil-to-fertilizer map:") {
            state = D5P1State::SoilToFertilizer;
        }
        else if line.starts_with("fertilizer-to-water map:") {
            state = D5P1State::FertilizerToWater;
        }
        else if line.starts_with("water-to-light map:") {
            state = D5P1State::WaterToLight;
        }
        else if line.starts_with("light-to-temperature map:") {
            state = D5P1State::LightToTemperature;
        }
        else if line.starts_with("temperature-to-humidity map:") {
            state = D5P1State::TemperatureToHumidity;
        }
        else if line.starts_with("humidity-to-location map:") {
            state = D5P1State::HumidityToLocation;
        } else {
            //println!("Parsing map entry: {}", line);
            d5p1_parse_map(&line, &mut tmp_ranges);
        }
    }

    assert_eq!(seeds.len()%2, 0);
    let mut tmp_src: Vec<D5P2Range> = vec![];
    for seed in seeds.chunks(2) {
        println!("Processing seed at: {} range: {}", seed[0], seed[1]);
        tmp_src.push(D5P2Range { src: seed[0].parse().unwrap(), len: seed[1].parse().unwrap() })
    }
    println!("Parsing SeedToSoil");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::SeedToSoil).unwrap());
    println!("Parsing SoilToFertilizer");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::SoilToFertilizer).unwrap());
    println!("Parsing FertilizerToWater");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::FertilizerToWater).unwrap());
    println!("Parsing WaterToLight");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::WaterToLight).unwrap());
    println!("Parsing LightToTemperature");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::LightToTemperature).unwrap());
    println!("Parsing TemperatureToHumidity");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::TemperatureToHumidity).unwrap());
    println!("Parsing HumidityToLocation");
    tmp_src = d5p2_process(&mut tmp_src, &states_lut.get(&D5P1State::HumidityToLocation).unwrap());
    println!("Parsing finished");
    for entry in tmp_src {
        if entry.src < result {
            result = entry.src;
        }
    }
    println!("Final result: {}", result);
}

fn day_6_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 6 part 1");
    let mut result = 1;

    let times: Vec<&str> = input_lines[0].split(' ').filter(|x| x.len() > 0).collect();
    let distances: Vec<&str> = input_lines[1].split(' ').filter(|x| x.len() > 0).collect();
    assert_eq!(times[0], "Time:");
    assert_eq!(distances[0], "Distance:");
    assert_eq!(times.len(), distances.len());
    for idx in 1..times.len() {
        let time: u64 = times[idx].parse().unwrap();
        let distance: u64 = distances[idx].parse().unwrap();
        println!("Parsing time: {} and distance: {}", time, distance);
        let valid: Vec<u64>= (1..time).filter(|x| x * (time - x) > distance).collect();
        println!("Possible times: {:?}", valid);
        result = result * valid.len();
    }
    println!("Final result: {}", result);
}

fn day_6_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 6 part 2");
    let mut result = 0;

    let times: Vec<&str> = input_lines[0].split(' ').filter(|x| x.len() > 0).skip(1).collect();
    let distances: Vec<&str> = input_lines[1].split(' ').filter(|x| x.len() > 0).skip(1).collect();
    let time: u64 = times.join("").parse().unwrap();
    let distance: u64 = distances.join("").parse().unwrap();
    println!("Count for time: {} and distance: {}", time, distance);
    for i in 1..time {
        if i * (time - i) > distance {
            result = time - i - i + 1;
            break;
        }
    }
    println!("Final result: {}", result);
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum D7P1Type {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct D7P1Hand {
    hand: String,
    name: D7P1Type,
    bid: u64,
}

fn d7p1_hand_to_type(input: &str) -> D7P1Type {
    let stats = input.to_string().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    println!("Preparsed {:?}", stats);
    if stats.len() == 5 {
        // 1 + 1 + 1 + 1 + 1
        return D7P1Type::HighCard;
    } else if stats.len() == 4 {
        // 2 + 1 + 1 + 1
        return D7P1Type::OnePair;
    } else if stats.len() == 3 {
        // 3 + 1 + 1, 2 + 2 + 1
        for (_, val) in stats.iter() {
            if *val == 2 {
                return D7P1Type::TwoPair;
            }
        }
        return D7P1Type::ThreeOfKind;
    } else if stats.len() == 2 {
        // 4 + 1, 3 + 2
        let tmp = stats.get(stats.keys().next().unwrap()).unwrap();
        if tmp == &4 || tmp == &1 {
            return D7P1Type::FourOfKind;
        }
        return D7P1Type::FullHouse;
    } else if stats.len() == 1 {
        return D7P1Type::FiveOfKind;
    }
    D7P1Type::HighCard
}

fn d7p1_compare(a: &D7P1Hand, b: &D7P1Hand) -> std::cmp::Ordering {
    let lut: HashMap<char, u8> = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    let lut_h: HashMap<D7P1Type, u8> = HashMap::from([
        (D7P1Type::HighCard, 0),
        (D7P1Type::OnePair, 1),
        (D7P1Type::TwoPair, 2),
        (D7P1Type::ThreeOfKind, 3),
        (D7P1Type::FullHouse, 4),
        (D7P1Type::FourOfKind, 5),
        (D7P1Type::FiveOfKind, 6),
    ]);
    if a.name == b.name {
        // hands are equal check cards starting from first
        for i in 0..a.hand.len() {
            let tmp_a = a.hand.chars().nth(i).unwrap();
            let tmp_b = b.hand.chars().nth(i).unwrap();
            if tmp_a == tmp_b {
                continue;
            }
            if lut.get(&tmp_a).unwrap() > lut.get(&tmp_b).unwrap() {
                return std::cmp::Ordering::Greater;
            }
            return std::cmp::Ordering::Less;
        }
    } else {
        if lut_h.get(&a.name).unwrap() > lut_h.get(&b.name).unwrap() {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Greater;
}

fn d7p1_parse_input(input: &String) -> D7P1Hand {
    let val: Vec<_> = input.split(' ').collect();
    return D7P1Hand { hand: val[0].to_string(), name: d7p1_hand_to_type(val[0]), bid: val[1].parse().unwrap() }
}

fn day_7_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 7 part 1");
    let mut result: u64 = 0;
    let mut hands: Vec<D7P1Hand> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        hands.push(d7p1_parse_input(line))
    }
    hands.sort_by(d7p1_compare);
    for (idx, hand) in hands.iter().enumerate() {
        println!("Parsed hand: {} {:?}", idx+1, hand);
        result += (idx as u64 + 1) * hand.bid;
    }
    println!("Final result: {}", result);
}

fn d7p2_hand_to_type(input: &str) -> D7P1Type {
    let stats = input.to_string().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    println!("Preparsed {:?}", stats);
    let js = stats.get(&'J');
    if stats.len() == 1 {
        return D7P1Type::FiveOfKind;
    } else if stats.len() == 5 {
        // 1 + 1 + 1 + 1 + 1
        if js.is_some() {
            return D7P1Type::OnePair;
        }
        return D7P1Type::HighCard;
    } else if stats.len() == 4 {
        // 2 + 1 + 1 + 1
        if js.is_some() {
            // Either 2J + 1X, else 1J + 2X
            return D7P1Type::ThreeOfKind;
        }
        return D7P1Type::OnePair;
    } else if stats.len() == 3 {
        // 3 + 1 + 1, 2 + 2 + 1
        if js.is_some() {
            let tmp = js.unwrap();
            for (key, val) in stats.iter() {
                if key == &'J' {
                    continue;
                }
                if *val == 2 {
                    if tmp == &2 {
                        return D7P1Type::FourOfKind;
                    } else {
                        return D7P1Type::FullHouse;
                    }
                }
            }
            return D7P1Type::FourOfKind;
        }
        for (_, val) in stats.iter() {
            if *val == 2 {
                return D7P1Type::TwoPair;
            }
        }
        return D7P1Type::ThreeOfKind;
    } else if stats.len() == 2 {
        // 4 + 1, 3 + 2
        let tmp = stats.get(stats.keys().next().unwrap()).unwrap();
        if js.is_some() {
            return D7P1Type::FiveOfKind;
        }
        if tmp == &4 || tmp == &1 {
            return D7P1Type::FourOfKind;
        }
        return D7P1Type::FullHouse;
    }
    D7P1Type::HighCard
}

fn d7p2_compare(a: &D7P1Hand, b: &D7P1Hand) -> std::cmp::Ordering {
    let lut: HashMap<char, u8> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    let lut_h: HashMap<D7P1Type, u8> = HashMap::from([
        (D7P1Type::HighCard, 0),
        (D7P1Type::OnePair, 1),
        (D7P1Type::TwoPair, 2),
        (D7P1Type::ThreeOfKind, 3),
        (D7P1Type::FullHouse, 4),
        (D7P1Type::FourOfKind, 5),
        (D7P1Type::FiveOfKind, 6),
    ]);
    if a.name == b.name {
        // hands are equal check cards starting from first
        for i in 0..a.hand.len() {
            let tmp_a = a.hand.chars().nth(i).unwrap();
            let tmp_b = b.hand.chars().nth(i).unwrap();
            if tmp_a == tmp_b {
                continue;
            }
            if lut.get(&tmp_a).unwrap() > lut.get(&tmp_b).unwrap() {
                return std::cmp::Ordering::Greater;
            }
            return std::cmp::Ordering::Less;
        }
    } else {
        if lut_h.get(&a.name).unwrap() > lut_h.get(&b.name).unwrap() {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Greater;
}

fn d7p2_parse_input(input: &String) -> D7P1Hand {
    let val: Vec<_> = input.split(' ').collect();
    return D7P1Hand { hand: val[0].to_string(), name: d7p2_hand_to_type(val[0]), bid: val[1].parse().unwrap() }
}

fn day_7_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 7 part 2");
    let mut result: u64 = 0;
    let mut hands: Vec<D7P1Hand> = vec![];
    for (idx, line) in input_lines.into_iter().enumerate() {
        println!("Parsing line {}: {}", idx, line);
        hands.push(d7p2_parse_input(line))
    }
    hands.sort_by(d7p2_compare);
    println!("Sorted: {:?}", hands);
    for (idx, hand) in hands.iter().enumerate() {
        if hand.hand.find('J').is_some() {
            println!("Parsed hand: {} {:?}", idx+1, hand);
        }
        result += (idx as u64 + 1) * hand.bid;
    }
    println!("Final result: {}", result);
}

fn day_8_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 8 part 1");
    let mut result = 0;

    let directions = input_lines.get(0).unwrap();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    println!("Directions: {}", directions);
    for (_,line) in input_lines.into_iter().skip(2).enumerate() {
        let tmp: Vec<&str> = line.split(' ').collect();
        assert_eq!(tmp.len(), 4);
        println!("{:?}", tmp);
        nodes.insert(tmp[0], (&tmp[2][1..tmp[2].len()-1], &tmp[3][..tmp[3].len()-1]));
    }
    println!("{:?}", nodes);
    let mut current = "AAA";
    while current != "ZZZ" {
        if directions.chars().nth(result%directions.len()).unwrap() == 'R' {
            current = nodes.get(current).unwrap().1;
        } else {
            current = nodes.get(current).unwrap().0;
        }
        result += 1;
    }
    println!("Final result: {}", result);
}

fn day_8_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 8 part 2");
    let mut result: u64;
    let mut results: Vec<u64> = vec![];

    let directions = input_lines.get(0).unwrap();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut currents: Vec<&str> = vec![];

    println!("Directions: {}", directions);
    for (_,line) in input_lines.into_iter().skip(2).enumerate() {
        let tmp: Vec<&str> = line.split(' ').collect();
        assert_eq!(tmp.len(), 4);
        println!("{:?}", tmp);
        nodes.insert(tmp[0], (&tmp[2][1..tmp[2].len()-1], &tmp[3][..tmp[3].len()-1]));
        if tmp[0].ends_with("A") {
            currents.push(&tmp[0]);
        }
    }
    println!("Nodes: {:?}", nodes);
    println!("Start: {:?}", currents);
    for mut current in currents {
        println!("Parsing: {}", current);
        result = 0;
        while !current.ends_with('Z') {
            if directions.chars().nth(result as usize%directions.len()).unwrap() == 'R' {
                current = nodes.get(current).unwrap().1;
            } else {
                current = nodes.get(current).unwrap().0;
            }
            result += 1;
        }
        results.push(result);
    }
    println!("Final result: {:?}", results);
    result = 0;
    for tmp in results {
        println!("Tmp result: {} {}", tmp, result);
        if result == 0 {
            result = tmp;
        } else {
            result = aoc_lcm(tmp, result);
        }
    }
    println!("Final result: {}", result);
}

fn d9p1_get_next_step(vals: &Vec<i64>) -> i64 {
    //println!("Values to parse: {:?}", vals);
    let mut diffs: Vec<i64> = vec![];
    let mut all_same = true;
    for idx in 1..vals.len() {
        let diff = vals[idx] - vals[idx-1];
        diffs.push(diff);
        if diffs.first().unwrap() != &diff {
            all_same = false;
        }
    }
    if all_same {
        return *vals.last().unwrap() + *diffs.first().unwrap();
    }
    return *vals.last().unwrap() + d9p1_get_next_step(&diffs);
}

fn day_9_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 9 part 1");
    let mut result = 0;

    for (idx ,line) in input_lines.into_iter().enumerate() {
        let series: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let next = d9p1_get_next_step(&series);
        result += next;
        println!("Idx: {} Next: {} Intermediate result: {}", idx, next, result);
    }
    println!("Final result: {}", result);
}

fn d9p2_get_prev_step(vals: &Vec<i64>) -> i64 {
    //println!("Values to parse: {:?}", vals);
    let mut diffs: Vec<i64> = vec![];
    let mut all_same = true;
    for idx in 1..vals.len() {
        let diff = vals[idx] - vals[idx-1];
        diffs.push(diff);
        if diffs.first().unwrap() != &diff {
            all_same = false;
        }
    }
    if all_same {
        return *vals.first().unwrap() - *diffs.first().unwrap();
    }
    return *vals.first().unwrap() - d9p2_get_prev_step(&diffs);
}

fn day_9_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 9 part 2");
    let mut result = 0;

    for (idx ,line) in input_lines.into_iter().enumerate() {
        println!("{}: {}", idx, line);
        let series: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let next = d9p2_get_prev_step(&series);
        result += next;
        println!("Idx: {} Next: {} Intermediate result: {}", idx, next, result);
    }
    println!("Final result: {}", result);
}

fn d10p1_is_valid_move(map: &Vec<String>, pos: (i16, i16)) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        return false;
    }
    if pos.0 >= map.first().unwrap().len() as i16 || pos.1 >= map.len() as i16 {
        return false;
    }
    true
}

fn d10p1_get_next(map: &Vec<String>, pos: &(i16, i16), visited: &HashSet<(i16, i16)>, nexts: &mut Vec<(i16, i16)>) {
    let steps: HashMap<char, Vec<(i16, i16)>> = HashMap::from([
        ('|', vec![(0, -1), (0, 1)]),
        ('-', vec![(-1, 0), (1, 0)]),
        ('L', vec![(0, -1), (1, 0)]),
        ('J', vec![(0, -1), (-1, 0)]),
        ('7', vec![(0, 1), (-1, 0)]),
        ('F', vec![(0, 1), (1, 0)]),
        ('.', vec![]),
        ('S', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    ]);

    let is_valid_nbr = |pos: &(i16, i16), valid_pos: &Vec<(i16, i16)>| -> bool {
        // check if pipe on next position is valid neighbor
        // println!("Validating nbr at: {:?} against: {:?}", pos, valid_pos);
        for tmp in valid_pos {
            if tmp.0 + pos.0 == 0 && tmp.1 + pos.1 == 0 {
                return true;
            }
        }
        return false;
    };
    println!("Getting nexts for {:?}", pos);
    let current_pipe = map[pos.1 as usize].chars().nth(pos.0 as usize).unwrap();
    assert!(steps.contains_key(&current_pipe));
    let next_steps = steps.get(&current_pipe).unwrap();
    for tmp in next_steps {
        let next_pos = (pos.0 + tmp.0, pos.1 + tmp.1);
        if !d10p1_is_valid_move(map, next_pos) {
            continue;
        }
        let next_pipe = map[next_pos.1 as usize].chars().nth(next_pos.0 as usize).unwrap();
        //println!("Checking: {} at {:?}/{:?}", next_pipe, next_pos, tmp);
        if is_valid_nbr(tmp, steps.get(&next_pipe).unwrap()) && !visited.contains(&next_pos) {
            nexts.push(next_pos);
        }
    }
}

fn day_10_part_1(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 10 part 1");
    let mut result = 0;
    let mut start = (-1, -1);

    for (idx ,line) in input_lines.into_iter().enumerate() {
        println!("{}: {}", idx, line);
        if line.contains('S') {
            start = (line.find('S').unwrap() as i16, idx as i16);
        }
    }

    let mut tmp_map = input_lines.clone();
    for line in tmp_map.iter_mut() {
        *line = std::iter::repeat('.').take(line.len()).collect::<String>()
    }

    println!("Starting at: {:?}", start);
    let mut nexts:Vec<(i16, i16)> = vec![start];
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    while !nexts.is_empty() {
        let next = nexts.pop().unwrap();
        tmp_map[next.1 as usize].replace_range(next.0 as usize..next.0 as usize +1, "#");
        visited.insert(next);
        d10p1_get_next(input_lines, &next, &visited, &mut nexts);
    }
    for line in tmp_map {
        println!("{:?}", line);
    }
    result = visited.len();
    println!("Visited nodes: {}", result);
    println!("Final result: {}", result/2);
}

fn day_10_part_2(input_lines: &Vec<String>) {
    println!("AoC 2023 Day 10 part 2");
    let result = 0;

    for (idx ,line) in input_lines.into_iter().enumerate() {
        println!("{}: {}", idx, line);
    }
    println!("Final result: {}", result);
}

fn main() {
    let solutions = HashMap::from([
        ("d1p1".to_string(), day_1_part_1 as fn(&Vec<String>) ), // cast to let compiler know about item
        // type, no need to do it for latter items
        ("d1p2".to_string(), day_1_part_2 ),
        ("d2p1".to_string(), day_2_part_1 ),
        ("d2p2".to_string(), day_2_part_2 ),
        ("d3p1".to_string(), day_3_part_1 ),
        ("d3p2".to_string(), day_3_part_2 ),
        ("d4p1".to_string(), day_4_part_1 ),
        ("d4p2".to_string(), day_4_part_2 ),
        ("d5p1".to_string(), day_5_part_1 ),
        ("d5p2".to_string(), day_5_part_2 ),
        ("d6p1".to_string(), day_6_part_1 ),
        ("d6p2".to_string(), day_6_part_2 ),
        ("d7p1".to_string(), day_7_part_1 ),
        ("d7p2".to_string(), day_7_part_2 ),
        ("d8p1".to_string(), day_8_part_1 ),
        ("d8p2".to_string(), day_8_part_2 ),
        ("d9p1".to_string(), day_9_part_1 ),
        ("d9p2".to_string(), day_9_part_2 ),
        ("d10p1".to_string(), day_10_part_1 ),
        ("d10p2".to_string(), day_10_part_2 ),
    ]);
    if env::args().count() != 3 {
        println!("Usage: program_name day_and_part input_path");
        println!("\tday_and_part - i.e. d1p1 for day 1 part 1");
        println!("\tinput_path - path to file with input data");
        return;
    }

    let day_part = env::args().nth(1).unwrap();
    let input_path = env::args().nth(2).unwrap();
    let input = read_to_string(input_path).unwrap().lines().map(String::from).collect();

    if !solutions.contains_key(&day_part) {
        println!("Invalid day or part: should be dXpY, where X is day number, and Y is part number");
        return;
    }
    solutions.get(&day_part).unwrap()(&input);
}

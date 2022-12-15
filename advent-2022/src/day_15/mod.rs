use std::{ io, collections::{HashMap} };

use itertools::Itertools;

use crate::utils::{get_lines};

fn get_distance(p1: (i128, i128), p2: (i128,i128)) -> i128 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

fn ranges_overlap(r1: (i128, i128), r2: (i128, i128)) -> bool {
    return r1.0 <= r2.1 && r2.0 <= r1.1
}

fn merge_ranges(
    ranges: &mut Vec<(i128,i128)>
) {
    if ranges.len() <= 1 { return }
    let mut merge = false;
    let mut idx1 = 0;
    let mut idx2 = 0;

    for _i in 0..ranges.len() {
        idx2 = 0;
        for _j in 0..ranges.len() {
            if idx1 != idx2 && ranges_overlap(ranges[idx1], ranges[idx2]) {
                merge = true; break;
            }
            idx2 += 1
        }
        if merge { break }
        idx1 += 1;
    }

    if merge {
        ranges[idx2] = (ranges[idx2].0.min(ranges[idx1].0), ranges[idx2].1.max(ranges[idx1].1));
        ranges.remove(idx1);
        merge_ranges(ranges);
    }
}

fn get_busy_intervals_on_y(
    sensor_vs_dist : &HashMap<(i128,i128), i128>,
    y : i128,
)  -> Vec<(i128, i128)> {
    let mut busy_ranges_y : Vec<(i128,i128)> = Vec::new();

    for sensor in sensor_vs_dist.keys() {
        // get range for y 
        let sensor_rad = *sensor_vs_dist.get(sensor).unwrap();
        let sensor_rad_on_y = (sensor_rad - (y - sensor.1).abs()).abs();

        if (sensor.1 - y).abs() <= sensor_rad {
            let local_range = (sensor.0 - sensor_rad_on_y + 0, sensor.0 + sensor_rad_on_y+0); 
            busy_ranges_y.push(local_range);

        }
    }

    merge_ranges(&mut busy_ranges_y);
    busy_ranges_y.sort_by(|a,b| a.0.cmp(&b.1));
    return busy_ranges_y;
}

fn count_busy( sensor_vs_dist : &HashMap<(i128,i128), i128>,y : i128 ) -> i128 {
    let busy_intervals = get_busy_intervals_on_y(sensor_vs_dist, y);
    return busy_intervals.iter().map(|x| x.1 - x.0).sum();
}

fn find_freq( 
    sensor_vs_dist : &HashMap<(i128,i128), i128>, 
    range : (i128,i128) 
) -> i128 {
    let mut pos : (i128, i128) = (0,0);

    for i in range.0..range.1+1{
        let busy_intervals = get_busy_intervals_on_y(sensor_vs_dist, i);
        for (i1, i2) in busy_intervals.iter().tuple_windows() {
            for free in i1.1+1..i2.0 {
                if free >= range.0 && free <= range.1 {
                    pos = (i,free);
                    return pos.1 * 4000000 + pos.0;
                }
            }
        }
    }
    return pos.1 * 4000000 + pos.0;
}

pub fn solve() -> Result<(), io::Error> {
    let mut sensor_vs_dist : HashMap<(i128,i128), i128> = HashMap::new();

    for line in get_lines("day_15") {
        let coordinates = line
            .replace("Sensor at x=", "")
            .replace(", y=", " ")
            .replace(": closest beacon is at x=", " ")
            .replace(", y=", " ").split(" ")
            .map(|x| x.parse::<i128>().unwrap())
            .collect_vec();

        sensor_vs_dist.insert(
            (coordinates[0], coordinates[1]),
            get_distance((coordinates[0], coordinates[1]), (coordinates[2], coordinates[3]))
        );
    }

    println!("First Star: {:?}", count_busy(&sensor_vs_dist, 2000000));
    println!("Second Star: {:?}", find_freq(&sensor_vs_dist, (0,4000000)));    

    return Ok(())
}
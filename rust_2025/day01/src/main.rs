use std::{fs};

pub enum Dir {
    L,
    R,
}

pub struct Rot {
    distance: i16,
    direction: Dir,
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");
    
    let input_vec = input.lines()
        .collect::<Vec<&str>>();

    let _rotations: Vec<Rot> = input_vec.iter()
        .map(|s| parse_rotation(s))
        .collect();

    
    let mut updated_distanceult = 50;
    let mut dial_1 = 0;
    let mut dial_2 = 0;

    for rotation in _rotations.iter() {
        updated_distanceult = rotate(updated_distanceult, rotation, &mut dial_2);

        if updated_distanceult == 0 {
            dial_1 += 1;
            dial_2 += 1;
        }
    }

    println!("Dial turned: {} times", dial_1);
    println!("Dial 2 turned: {} times", dial_2);
    
    let div_euclid = (-101i16).div_euclid(100);
    println!("Div euclid: {}", div_euclid);
}

fn parse_rotation(s: &str) -> Rot {
    let direction = match &s[0..1] {
        "L" => Dir::L,
        "R" => Dir::R,
        _ => panic!("Invalid direction"),
    };

    let distance: i16 = s[1..].parse()
        .expect("Failed to parse distance");

    Rot { distance, direction }
}

fn rotate(curr_distance: i16, rotation: &Rot, _dial_2: &mut i16) -> i16 {
    let n_dials = calculate_passed_zero_dials(curr_distance, rotation);
    *_dial_2 += n_dials;

    let mut updated_distance = update_distance(curr_distance, rotation);
    let update_distance1 = updated_distance.rem_euclid(100);

    updated_distance = updated_distance % 100;

    if updated_distance < 0 {
        updated_distance += 100;
    }

    assert_eq!(update_distance1, updated_distance);
    updated_distance
    
}

fn calculate_passed_zero_dials(original_distance: i16, rotation: &Rot) -> i16 {
    let mut updated_distance = update_distance(original_distance, rotation) % 100;

    if updated_distance < 0 {
        updated_distance += 100;
    }

    let mut count_zero = rotation.distance.div_euclid(100);

    match rotation.direction {
        Dir::L => {
            if original_distance < updated_distance && original_distance != 0 && updated_distance != 0 {
                count_zero += 1;
            }
        },
        Dir::R => {
            if original_distance > updated_distance && original_distance != 0 && updated_distance != 0 {  
                count_zero += 1;
            }
        },
    }

    count_zero
}

fn update_distance(curr_distance: i16, rotation: &Rot) -> i16 {
    match rotation.direction {
        Dir::L => curr_distance - rotation.distance,
        Dir::R => curr_distance + rotation.distance,
    }
}
use std::fs::File;
use std::io::prelude::*;
extern crate rand;
use std::f32::consts::E;
use rand::Rng;
use std::collections::HashMap;


pub struct Airport {
    ID:u32,
    allto : Vec<u32>,
}
fn main(){
    let filename="routes.txt";
    let file=File::open(filename).unwrap();
    let lines=std::io::BufReader::new(file).lines();
    let mut all_airport:Vec<Airport> = Vec::new();
    let mut all_id:Vec<u32> = Vec::new();
    let mut indexid = 0;
    for line in lines{
        let str=line.unwrap();

        let mut number:Vec<&str> =str.split(",").collect();
        let mut airport_id:u32;
        let mut to:u32;
        if number[3] == "N"{
            //println!("==============");
            let mut char_vec: Vec<char> = number[2].chars().collect();
            airport_id = (char_vec[0].to_digit(36).unwrap())*10000+ (char_vec[1].to_digit(36).unwrap())*100+(char_vec[2].to_digit(36).unwrap());//for thus airport without a ID, give them an unique id by their name's character
            //println!("{}",airport_id);
        }
        else{
            airport_id = number[3].parse::<u32>().unwrap();
            //println!("{}",airport_id);
        }
        if number[5] == "N"{
            //println!("==============");
            let mut char_vec: Vec<char> = number[4].chars().collect();
            to = (char_vec[0].to_digit(36).unwrap())*10000+ (char_vec[1].to_digit(36).unwrap())*100+(char_vec[2].to_digit(36).unwrap());
            //println!("{}",to);
        }
        else{
            to = number[5].parse::<u32>().unwrap();
            //println!("{}",to);
        }

        if all_id.contains(& airport_id){
            let mut index = all_id.iter().position(|&x| x == airport_id).unwrap();

            if all_airport[index].allto.contains(&to) == false{
                all_airport[index].allto.push(to);
            }
        }
        else{
            let mut new_airport = Airport{
                ID:airport_id,
                allto:vec![to],
            };
            all_airport.push(new_airport);
            all_id.push(airport_id);
        
        }
    }
    let num = all_id.len();
    let mut routes = HashMap::new();
    for i in 0..num{
        let mut x = &all_airport[i];
        routes.insert(x.ID,&x.allto);
    }//make it graph

    let mut total_num:u32 = 0;
    for i in 0..100{
        let mut rng = rand::thread_rng();
        let mut start:u32 = all_id[rng.gen_range(0..num)];
        let mut have:Vec<u32> = Vec::new();
        have.push(start);
        if let Some(can) = &routes.get(&have[0]) {
            for t in 0..can.len(){
                have.push(can[t]);
            }
        }
        for j in 1..have.len(){
            if let Some(can) = &routes.get(&have[j]) {
                for t in 0..can.len(){
                    if have.contains(&can[t]) == false{
                        have.push(can[t]);
                    }
                }
            }
        }
        total_num = total_num + have.len() as u32;
        //println!("{}",have.len());
        
    }
    let mut average_number_canget = total_num/100;
    
    println!("the average number airports you can get to within 2 transfer is {}",average_number_canget);


    let mut total_num_move:u32 =0;
    let mut too_far:u32 = 0;
    for m in 0..100{
        let mut path:u32 =0;
        let mut rng = rand::thread_rng();
        let mut start:u32 = all_id[rng.gen_range(0..num)];
        let mut target:u32 = all_id[rng.gen_range(0..num)];
        let mut have:Vec<u32> = Vec::new();
        have.push(start);
        while have.contains(&target) == false{
            path = path + 1; 
            for j in 0..have.len(){
                if let Some(can) = &routes.get(&have[j]) {
                    for t in 0..can.len(){
                        if have.contains(&can[t]) == false{
                            have.push(can[t]);
                        }
                    }
                }
            }
            if path > 10{
                too_far = too_far +1;
                break
            }
        }
        //println!("{}",path);
        if path < 11{
        total_num_move = total_num_move + path;}

    }
    let mut average_num_move:f32 = total_num_move as f32 /(100.0-too_far as f32);
    let mut p_unable:f32 = (too_far as f32)/100.0;
    println!("the average transfer needed to travel from a airport to another is{}, while {} of airport may be very hard or unable to reach from some airport",average_num_move,p_unable);
}




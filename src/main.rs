use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::time::Duration;
use std::{vec, usize};
use regex::Regex;
use time::PrimitiveDateTime;
use std::cmp::Ordering;
fn after(start:PrimitiveDateTime,time:&u64) ->PrimitiveDateTime{
    start+Duration::new(time*3600, 0)
}
// fn time_iter(  &input_vec:Vec<String>,start:Option<PrimitiveDateTime>, end:PrimitiveDateTime){
//         // let mut datetimecout:Vec<String>=Vec::new();
//         if let Some(smallest_time)=start{
//             //println!("Smallest time is from the function :{}",smallest_time);
//             if end > smallest_time{
//                 //println!("Parsed time from the function :{:?}",end);
//                 input_vec.push(end.to_string());
//                 println!("Datetime vector:{:?}",input_vec);
//             }
            
//         }
//         else {
//             println!("Not match for this case")
//         }
//     }



// #[derive(Debug, Clone, Copy)]
// struct AddTime<T>{
//     value: T
// }

// impl<T:Copy+std::ops::AddAssign,R:Into<AddTime<T>>> std::ops::AddAssign<R>
// for  AddTime<T>
// {
//    fn add_assign(&mut self, rhs: R) {
//        let rhs=rhs.into();
//        self.value+=rhs.value
//    } 
// }
struct AddNothing{
    input:PrimitiveDateTime,
    current:PrimitiveDateTime,
    output:PrimitiveDateTime,
}
#[derive(Debug)]
struct AddTime{
    smallest_time:String,// for testing use String type only but for actual use PrimitiveDateTime
    added_time:String,
    parsed_time:String,

}
fn line_count(ln_str:String,  line:&mut std::io::Lines<BufReader<File>>,mut collect_map:HashMap<String,i32>){
    println!("Header line:{:?}",ln_str);
    if let Some(next_line) =line.next()  {
        
        println!("sequence line:{:?}",next_line);
        if let Ok(next_line_result) =next_line  {
            let line_length: i32=next_line_result.len().try_into().unwrap();
            println!("Sequence line length:{:?}",line_length);
            let barcode_name=barcode_extraction(ln_str);
            collect_map.insert(barcode_name.to_string(), line_length);
            
        }else if let Err(err)=next_line {
            eprintln!("Error reading line:{:?}",err);
            
        };
        
    }
    if let Some(after_next_line) =line.next()  {
        println!("Plus line:{:?}",after_next_line);
    }
    if let Some(fourth_line) =line.next()  {
        println!("QC line:{:?}",fourth_line);
    }
}
fn barcode_extraction(lin_str:String)-> String{
    let barcode_re=Regex::new(r"barcode=(?P<barcode>\S+)\s*").unwrap();
    if let Some(cap_barcode) = barcode_re.captures(&lin_str) {
        let barcode_str=cap_barcode.name("barcode").unwrap().as_str().to_string();
        // println!("barcode string part:{}",barcode_str);
        return barcode_str;
    }
    // barcode_str
}

fn main() -> std::io::Result<()> {
    let file = File::open("new.fastq")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut smallest_datetime: Option<PrimitiveDateTime> = None;
    //let mut smallest_datetime = AddTime { value: PrimitiveDateTime::now() };
    let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
    let mut date_time_take:Vec<String>=Vec::new();
    let mut Collect_barcode:HashMap<String,i32>=HashMap::new();
    while let Some(line) = lines.next() {
        let line_str = line?;
        if let Some(captures) = date_time_re.captures(&line_str) {
            let datetime_str = captures.name("time").unwrap().as_str();
            let sliced_datetime = &datetime_str[..19];
            if let Ok(parsed_datetime) = PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
                // println!("Date time frame: {}", parsed_datetime);
                //getting smallest date time
                if let Some(smallest) = smallest_datetime {
                    if parsed_datetime < smallest {
                        smallest_datetime = Some(parsed_datetime);
                        // println!("Parsed time for every frame:{}",parsed_datetime);
                        println!("Smallesst date time frame ");
                        // line_count(line_str.clone(), &mut lines,Collect_barcode);
                        // println!("Header line is: {:?}", line_str);
                        // if let Some(next_line) = lines.next() {
                        //     println!("Sequence line is: {:?}", next_line?);
                        // }
                        // if let Some(after_next_line) = lines.next() {
                        //     println!("Plus line: {:?}", after_next_line?);
                        // }
                        // if let Some(fourth_line) = lines.next() {
                        //     println!("QC line: {:?}", fourth_line?);
                        // }
                        // smallest+=parsed_datetime
                        // println!("smallest time inside the loop:{:?}",smallest_datetime);
                        let poke_time=after(smallest, &3);
                        // println!("Poke time inside the loop:{}",poke_time);
                        let point=AddTime{smallest_time:smallest.to_string(),added_time:poke_time.to_string(),parsed_time:parsed_datetime.to_string()};
                        // println!("Add struct inside the while loop:{:?}",point);
                        
                    }
                    //println!("Smallest time is outside the box:{:?}",smallest_datetime);
                    date_time_take.push(parsed_datetime.to_string());
                    // println!("Date time take:{:?}",date_time_take);

                } else {
                    smallest_datetime = Some(parsed_datetime);
                }





                
                //new scope for comparing datetimeframe
                if let Some(new) =smallest_datetime  {
                let test_time=after(new, &7);
                if parsed_datetime< test_time{
                    println!("Parsed time within the range:{}",parsed_datetime);
                    line_count(line_str.clone(),&mut lines,Collect_barcode);
                    
                }
                    
                }
            }
        }
        
    }
    // println!("Date time taking vector without sort:{:?}",date_time_take);
    println!("Collect barcode :{:?}",Collect_barcode);
    
    

    if let Some(smallest) = smallest_datetime {
        let added_time=after(smallest,&3);
        println!("Smallest datetime frame: {}", smallest);
        println!("New added time is :{}",added_time);
        // let point=AddTime{smallest_time:smallest.to_string(),added_time:added_time.to_string()};
        // println!("Struct field value :{:?}",point);
        
    } else {
        println!("No datetime frames found in the file.");
    }

    Ok(())
}

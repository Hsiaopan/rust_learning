use std::io;

mod conversions;

fn main() {
    println!("Choose a target.");
    loop {
        let mut target_unit = String::new();
        io::stdin()
            .read_line(&mut target_unit)
            .expect("Failed to read line");

        if "F".eq(target_unit.trim()) || "C".eq(target_unit.trim()) {
            
            println!("Please input the temperature.");
            let mut source_measure = String::new();
            io::stdin()
                .read_line(&mut source_measure)
                .expect("Failed to read line");
            
            let source_measure: f32 = match source_measure.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("{}{}",conversions::target_unit_convert(source_measure, &target_unit), target_unit);
            break;
        }else{
            println!("Please choose the right one");
            continue;
        }
    }
}

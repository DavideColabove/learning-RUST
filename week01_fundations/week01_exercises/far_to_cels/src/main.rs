use std::io;

fn main() {
    let mut farh = String::new(); 
    
    loop{
        io::stdin().read_line(&mut farh)
        .expect("Failed to read the line!");

        let farh : f32 = match farh.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("The conversion of {}°F to Celsius is:  {}°C", farh, far_to_cels(farh));
    }

    

}
 
fn far_to_cels(mut f: f32) -> f32{
    f =  f - 32.0;
    f = f * 5.0;
    f = f / 9.0;
    f
}


use std::io;

fn main() {
    
        println!("\n\n\t\t\tTemperature Converter\n\n");

        println!("To convert Celsius into Fahrenheit enter 1 or 2 to convert Fahrenheit into Celsius = ");
        let mut temp=String :: new();
        io::stdin().read_line(&mut temp)
        .expect("Failes to read line");
        let temp : i32= temp.trim().parse()
            .expect("please type number!");
    
if temp == 2 
        {            
            println!("Please Enter the Temperature in Farenheit = ");
            let mut fah=String::new();
            io::stdin().read_line(&mut fah)
                .expect("Failed to read line");
                let fah : i32=fah.trim().parse()
                       .expect("please type number!");
                
                let calc1= (fah-32) * 5/9;
                println!("\nThe {}°F in Celsius is = {}\n",fah,calc1);
        
        }

else if temp== 1
        {
            
                println!("Please Enter the Temperature in Celsius = ");
            let mut cel=String::new();
            io::stdin().read_line(&mut cel)
                .expect("Failed to read line");
                let cel : i32= cel.trim().parse()
                        .expect("please type number!");
                
                let calc2= (cel * 9/5) + 32;
                println!("\nThe {}°C in Fahrenheit is = {} \n",cel,calc2);
            
        }
    

}
        


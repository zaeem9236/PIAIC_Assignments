mod addition{
    pub mod add{
       pub fn sum(number_1: u8, number_2: u8) -> u8{
            number_1+number_2
        }
    }
}

fn main() {
    use crate::addition::add::sum;
    println!("Sum of numbers are {}",sum(3,2));
}
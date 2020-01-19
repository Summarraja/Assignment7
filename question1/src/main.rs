mod math{
    pub mod calculate{
        pub fn max(num1:isize , num2:isize) ->isize{
            if num1>num2{
                num1
            }else{
                num2
            }
        }
    }
}
//use math::calculate;

fn main() {
    println!("Maximum number is {}",math::calculate::max(88,55));
}

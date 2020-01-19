
pub mod fun{
    pub mod tool{
        pub fn lucky_number(name: String)-> u8{

            let vector = name.clone().to_lowercase().into_bytes();
            let mut num=0;
            //println!("{:?}",vector);
            for ch in vector{
                if ch>=97 && ch<=122{
                    num+=ch-96;
                    num%=9;
                }                  
            }
            if num==0{
                num=9;
            }
            num
        }
    }
}

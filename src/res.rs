pub mod hel{
    pub mod hi {
       pub fn say_name(name: &mut &str){
          println!("my name is {}", name);
          *name= "hello";
       }
    }
 }
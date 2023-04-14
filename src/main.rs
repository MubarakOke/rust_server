
use crate::res::hel::hi::say_name;
use std::io;  
mod res;
use crate::Food::Red;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::fs::remove_file;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::{io::{Write, Read, Error}, fs::OpenOptions};
use std::io;
#[allow(unused_macros)]
macro_rules! my_macro {
    () => {
        println!("awesome macro")
    };
}

#[allow(unused_macros)]
macro_rules! my_macro1 {
   ($($val: expr),*) => {
       $(println!("{}", $val);)*
   };
}

#[allow(unused_macros)]
macro_rules! my_macro2 {
    (x=>$e:expr) => {
     println!("x macro {}", $e)   
    };
    (y=>$f:expr)=>{
      println!("y macro {}", $f) 
    }
}

#[allow(unused_macros)]
macro_rules! my_macro3 {
    (x=> $func: ident) => {
        fn $func(){
         println!("x type is called")
        }
    };
    (y=> $func: ident) => {
      fn $func(val: i32){
       println!("this is the value called: {}", val)
      }
  };
}

#[derive(Debug)]
struct Animal{
   #[allow(dead_code)]
   name: String,
   #[allow(dead_code)]
   age: u32,
   #[allow(dead_code)]
   breed: String
}

impl Animal{
   #[allow(dead_code)]
   fn bark(&self){
      println!("woof woof");
   }
   #[allow(dead_code)]
   fn wag(&self){
      println!("waging tail");
   }
}

trait Human {
   fn new(name: String, age: u32, skin: String)-> Self;
   fn talk(&self);
   fn sing(&self){
      println!("human singing");
   }
   fn random(&self, some: String){
      println!("random {}", some);
   }
   fn die(){
      println!("All humans die")
   }

}

trait Che {
   fn talk(&self);
   fn sing(&self){
      println!("human singing");
   }
}

#[derive(Debug)]
struct Baby{
   #[allow(dead_code)]
   name: String,
   #[allow(dead_code)]
   age: u32,
   #[allow(dead_code)]
   skin: String
}

impl Human for Baby {
   fn new(name: String, age: u32, skin: String)-> Self {
       Baby { name, age, skin}
   }

   fn talk(&self) {
       println!("Baby is talking")
   }
}
impl Che for Baby {
   fn talk(&self) {
       println!("Baby is talking")
   }
}

#[derive(Debug)]
struct Youth{
   #[allow(dead_code)]
   name: String,
   #[allow(dead_code)]
   age: u32,
   #[allow(dead_code)]
   skin: String
}

impl Human for Youth {
   fn new(name: String, age: u32, skin: String)-> Self {
       Youth { name, age, skin}
   }

   fn talk(&self) {
       println!("Youth is talking")
   }

   fn random(&self, some: String){
      println!("youth random {}", some);
   }

   fn sing(&self) {
      println!("Youth is singing");
   }

   fn die() {
      println!("Youth dies");
   }
}

impl Che for Youth {
   fn talk(&self) {
       println!("youth is talking")
   }
}

impl Youth {
   #[allow(dead_code)]
   fn beat(&self) {
      println!("Youth beats");
   } 
}

#[allow(dead_code)]
fn get_youth<T: Che + 'static>(you: T)-> Box<dyn Che>{
   Box::new(you)
}

#[allow(unused_attributes)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_attributes)]
#[allow(unused_mut)]

fn main() {
   let mut file= File::create("./src/check.txt").expect("error oooooo");
   let mut file= OpenOptions::new().append(true).open("./src/check.txt").expect("unabale to open");
    file.write_all("\thiii and helllo \n".as_bytes()).expect("unable to write")
   let mut file= File::open("./src/check.txt").unwrap();
   let mut content= String::new();
   file.read_to_string(&mut content).expect("unable to read from file");
   println!("{}", content);
   fs::remove_file("./src/check.txt").expect("unable to remove file");
   let mut file= File::open("./src/check.txt").unwrap();
   let b= display_file(&mut file);
   let b= display_file_no_inp();
   match b {
      Ok(st)=> println!("{}", st),
       Err(e)=> println!("{}",e)
   }
   // println!("{:?}", b);

   let my_str= "hello";
   let my_dat= [1,2,3,4];
   println!("{:?}", check_num(my_dat));
   let my_data2= ["hello", "hi", "peace", "gone"];
   let my_vec= vec![1,2,3,4,5];
   let b= my_vec;
   println!("{:?}", my_dat);
   let youth= Youth::new("youth akin".to_string(), 6, "red".to_string());
   let baby= Baby::new("baby akin".to_string(), 26, "black".to_string());
   println!("{:?}", get_youth(youth));
   let youth= Youth{name:"youth akin".to_string(), age:6, skin: "red".to_string()};
   get_youth(youth);
   youth.talk();
   youth.sing();
   youth.beat();
   baby.talk();
   baby.sing();
   youth.random("hello".to_string());
   baby.random("hi".to_string());
   Youth::die();
   youth.random("something".to_string());
   println!("{:?}", baby);  


   let my_animal1= Animal{name:"lucky".to_string(), age:5, breed: "chiwawa".to_string()};
   let my_animal2= Animal{name:"freggie".to_string(), age:7, breed: "rotweiler".to_string()};

   println!("{:?}", my_animal1);
   println!("{:?}", my_animal2);
   my_animal1.bark();
   my_animal1.wag();
   check_num(my_dat);
   let c= &my_dat;
   println!("{:?}", c);
   println!("{:?}", my_dat);

   -----------------------macros--------------------------
   my_macro!();
   my_macro1!("hello", "hi", "what's up");
   my_macro2!(x => "hello");
   my_macro2!(y => "hi");
   my_macro3!(x => hello);
   my_macro3!(y => hi);
   hello();
   hi(3);

   ---------------------------hof---------------------------
   let my_result1= cube(is_even, 4);
   let my_result2= cube(is_even, 3);
   println!("result {}", my_result1);
   println!("result {}", my_result2);

   ---------------------------closures-----------------------------
   let my_close= |x| {x};
   println!("my square is {}", my_close(3));

   -----------------------------for-----------------------------
   for i in my_dat{
      if i == 1{
         println!("good morning");
      }
      else {
         println!("just passing by");
      }
   }
   for (index,i) in (10..15).enumerate() {
      println!("{} val {}", index, i)
   }

   -----------------------matches-------------------------
   let my_tuple= (6, 5, 6);
   match my_tuple {
      (1,1,1)=> println!("thripple 1"),
      #[allow(unreachable_patterns)]
      (x,y, 1)=> println!("x {0} y 1{1} 1", x,y),
      #[allow(unreachable_patterns)]
      (1, 1, y)=> println!("x 1 y {}",y),
      #[allow(unreachable_patterns)]
      (x,1, 1)=> println!("x 1 y {}", x),
      #[allow(unreachable_patterns)]
      (x,y,z)=>println!("{} {} {} tuplesssss", x,y,z),
      #[allow(unreachable_patterns)]
      _=> println!("na tupple")
   }
   result(my_dat);
   --------------------------if-----------------------------
   let my_bool= if my_dat == "hello" {true} else {false};
   println!("{}", my_bool);
   let my_tup= vec![1,2,3];
   --------------------------struct----------------------------
   let emp = Employee {
      name:"hello".to_string(), 
      company: "hi".to_string(), 
      age: 100
   };
   let my_enum= Food::Red;
   println!("{:?}",emp);
}

#[allow(dead_code)]
fn display_file_no_inp()-> Result<String, io::Error>{
   let mut file= File::open("./src/check.txt")?;
   let mut container= String::new();
   file.read_to_string(&mut container)?;
   Ok(container)
}

#[allow(dead_code)]
fn display_file(f: &mut File)-> Result<String, io::Error>{
   let mut container= String::new();
   f.read_to_string(&mut container)?;
   Ok(container)
}

#[allow(dead_code)]
fn check_num<const T: usize>(val: [i32;T]){
   println!("{:?}", val);
}
#[allow(dead_code)]
fn check_num(val: [i32;4]){
   println!("{:?}", val);
   
}
#[allow(dead_code)]
fn check_num2<const T: usize>(val: [i32;T])->[i32;T]{
   val
   
}

#[allow(dead_code)]
fn check_num<const N: usize>(val: &[i32; N]){
   println!("{:?}", val);
   // let arr= [1; N];
   // arr
   
}
#[allow(dead_code)]
fn check_num(val: Vec<i32>){
   println!("{:?}", val);
}
#[allow(dead_code)]
fn cube(func: fn(i32)->bool, val: i32)-> i32{
   let check:bool = func(val);
   if check {
      return val* val*val;
   }
   else {
      return 0;
   }
}

#[allow(dead_code)]
fn is_even(val:i32)->bool{
   if val % 2==0{
      true
   }
   else{
      false
   }
}

#[allow(dead_code)]
fn result(val: i32){
   match val {
       1 => {println!("equal 1")},
       2 => {println!("equal 2")},
       _ if(val%2 ==0) => {println!("even number")}
       _ => {println!("not 1 nor 2")},
   }
}

#[derive(Debug)]
struct Employee<T, Y> {
   #[allow(dead_code)]
   name: T,
   #[allow(dead_code)]
   company: T,
   #[allow(dead_code)]
   age: Y
}

#[derive(Debug)]
enum Food {
   #[allow(dead_code)]
   Red,
   #[allow(dead_code)]
   Blue,
   #[allow(dead_code)]
   Gren
}
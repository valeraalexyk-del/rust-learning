// Задание 1
fn main(){
    let a =15;
    let mut b = 5;
    println!("{} {}", a , b);
    b = 20;
    println!("{} {}" , a , b);

// Задание 2

    let name = "Anton";
    let mut message = "hello".to_string();
    message.push_str(" ");
    message.push_str(name);
    
    println!("{}" , message);
   

}
// ----------------------Match Control Flow------------------
// -------------Basic Pattern matching Example ------------
// fn main(){
// let pm = "Nawaz Sharif";
//     match pm {
//        "Nawaz Sharif" => println!("Mujhe kyu nikala?"),
//        "Asif Zardari" => println!("Mei inko rulaonga!"),
//        "Bilawal Bhutto" => println!("Jab Barish ata hai ! toh pani ata hai"),
//        _ => println!("Tabdeli ayi hai!"),
//     }
// }



// Enum Example with match control flow  -----------------//
enum Days{
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
fn check_weekday(day:Days)-> u8 {
 match day {
     Days::Monday => { 
         println!("This is Weekday");
         1
     },
     Days::Tuesday => {
        println!("This is Weekday");
        2
     } ,
     Days::Wednesday => {
        println!("This is Weekday");
        3
     } ,
     Days::Thursday => {
        println!("This is Weekday");
        4
     } ,
     Days::Friday => {
        println!("This is Weekday");
        5
     } ,
     Days::Saturday => {
        println!("This is Weekend");
        6
     } ,
     Days::Sunday => {
        println!("This is Weekend");
        7
     } ,
 }
}
fn main(){
        let check_day = Days::Tuesday;
        let result = check_weekday(check_day);
        println!("{}",result);

        let check_day2 = Days::Saturday;
        let result2 = check_weekday(check_day2);
        println!("{}",result2);

}
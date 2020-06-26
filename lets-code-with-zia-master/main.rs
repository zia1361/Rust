#[macro_use]
extern crate mysql;
use mysql as my;
use std::io;


fn main() {

    println!("<<<<<=========================WELCOME QUARTER1 BATCH 9-12========================>>>>>");
    println!("
            1: LOGIN\n
            2: SIGNUP \n
            3: UPDATE \n
            4: DELETE");
            let mut response = String::new();
            io::stdin().read_line(&mut response).expect("failed to read");
            let response:i32 = response.trim().parse().expect("failed to convert");
           if response == 1{
                            println!("enter id");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("failed to read");
     let response = fetch(&id);
     println!("{:?}",response);
           } 
           if response == 2{
           create();
           }
           if response == 3{
               update();
           }
           if response == 4{
               delete();
           }
}
#[derive(Debug)]
struct Student {
    sid: i32,
    sname: Option<String>,
    semail: Option<String>,
    sage: Option<String>,
}
fn create(){
    println!("enter student id");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("failed to read");
    let id:i32 = id.trim().parse().expect("failed to convert");
    println!("enter student name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read");
    println!("enter student email");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("failed to read");
    println!("enter student age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read");
    
     let pool = my::Pool::new("mysql://root@localhost:3306/test").unwrap();


    let students = vec![
        Student { sid: id, sname:Some(name), semail: Some(email),sage: Some(age) }
    ];

    for mut stmt in pool.prepare("INSERT INTO tblstudent
                                       (sid, sname, semail,sage)
                                   VALUES
                                       (:sid, :sname, :semail, :sage)").into_iter() {
        for s in students.iter() {
            stmt.execute(params!{
                "sid" => s.sid,
                "sname" => &s.sname,
                "semail" => &s.semail,
                "sage"=> &s.sage,
            }).unwrap();
        }
    }
}
fn fetch(id:&String)->Vec<Student>{
    let query = String::from("SELECT sid, sname, semail, sage from tblstudent where sid =") + &id.to_string();
    
         let pool = my::Pool::new("mysql://root@localhost:3306/test").unwrap();
  let selected_student: Vec<Student> =
    pool.prep_exec(query, ())
    .map(|result| { 
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, email,age) = my::from_row(row);
            Student {
                sid: id,
                sname: name,
                semail: email,
                sage: age,
            }
        }).collect() 
    }).unwrap();
    selected_student

}
fn update(){
    println!("enter student id you want update");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("failed to read");
    let _response = 
    println!("enter new id");
    let mut new_id = String::new();
    io::stdin().read_line(&mut new_id).expect("failed to read");
    let _parse_id:&str = &id;
    println!("enter student name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read");
    println!("enter student email");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("failed to read");
    println!("enter student age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read");
    let query1 = String::from("UPDATE tblstudent SET sid='".to_owned()+&new_id.trim()+"',sname ='"+&name.trim());
    let query2 = String::from("', semail='".to_owned()+&email.trim()+"',sage='"+&age.trim()+"' where sid='"+&id.trim()+"';");
    let final_query = query1.clone()+&query2;
    println!("{}",&final_query);
    let pool = my::Pool::new("mysql://root@localhost:3306/test").unwrap();
    pool.prep_exec(final_query,()).unwrap();
}
fn delete(){
    println!("enter student id");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("failed to read");
    let query = "DELETE from tblstudent
     WHERE sid = '".to_owned()+&id.trim()+"'";      
     let pool = my::Pool::new("mysql://root@localhost:3306/test").unwrap();
    pool.prep_exec(query,()).unwrap();
}
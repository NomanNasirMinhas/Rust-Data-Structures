use std::fs::File;
use std::collections::HashMap;

struct Cal_Arr{
    arr: [u32;10]
}

impl Cal_Arr{
    fn sum(&self) -> u32{
        let it = self.arr.iter();
        let mut sum = 0;

        for i in it{
            sum = sum + i;
        }
        sum
    }

    fn avg(&self) -> u32{
        let sum = self.sum();
        let mut num = 0;
        for _i in self.arr.iter(){
            num = num + 1;
        }

        sum/num
    }
}

fn main() {
    println!("Hello, Welcome to Data Structures!\n--------------------------\n");
    let arr1 = [1,2,3,4,5,6,7];
    let arr2:[String; 3] = ["Noman".to_string(), "Nasir".to_string(), "Minhas".to_string()];
    let arr3 = ["10 for 10 Times"; 10];

    for i in &arr3{
        println!("{}", i);
    }

    let it = arr1.iter();

    for j in it{
        println!("{}", j);
    }

    let it2 = arr2.iter().rev();

    for k in it2{
        println!("{}", k)
    }


    /////////////////////////////////TUPLES///////////////////////
    let tup1 = (1, "Noman", 3.21);
    let tup2: (i32, i32, f32) = (1,33, 3.21);



    ///////////////////VECTORS/////////////////////
    let mut vec1 = vec![]; ///All RULES OF ARRAYS except is uses Stacks
    vec1.push("Hello");
    vec1.push("Noman");

    //////////////////////STRUCT/////////////////////
    let strct1 = Cal_Arr{
        arr:[12,4,22,53,11,8,66,77,12,21]
    };

    println!("Sum of the Array is {} and Average is {}", strct1.sum(), strct1.avg());



    //////////////////////////HASH MAPS/////////////////////////////

    let mut psl_teams = HashMap::new();

    psl_teams.insert("LQ", "Lahore Qalanders");
    psl_teams.insert("KK", "Karachi Kings");
    psl_teams.insert("MS", "Multan Sultans");
    psl_teams.insert("PZ", "Peshawar Zalmis");
    psl_teams.insert("QG", "Quetta Gladiators");
    psl_teams.insert("IU", "Islamabad United");

    let key = "LQ";
    // println!("LQ means {}", psl_teams.get("LQ")); Throws Error
    // println!("LQ means {}", psl_teams.get(&key)); Throws Error
    println!("LQ means {:?}", psl_teams.get(&key));

    println!("Iterating Over Hash Map");

    for (k,v) in &psl_teams{
        println!("Key = {} and Value = {}", k,v);
    }


    //////////////////////////////ENUMS///////////////////////////
    enum Exprtise_Level{
        Beginner,
        Intermediate,
        Pofessional,
        Expert
    }

    let expertise = Exprtise_Level::Beginner;
    // println!("Expertise Level is {}", expertise);  This wil give error
    // println!("Expertise Level is {}", expertise as u8);

    match expertise{
        Exprtise_Level::Beginner =>{
            println!("Expertise Level is Beginner");
        }

        Exprtise_Level::Intermediate =>{
            println!("Expertise Level is Intermediate");
        }

        Exprtise_Level::Pofessional =>{
            println!("Expertise Level is Pofessional");
        }

        Exprtise_Level::Expert =>{
            println!("Expertise Level is Expert");
        }
    }


    let f = File::open("./file.txt");

    let f = match f {
        Ok(file)=> file,
        Err(err)=> panic!("Error In Opening File =  {}",err)
    };

    let f = File::open("./file.txt").expect("Showing Expect Error");
    let f = File::open("./file.txt").unwrap();

    /////////////////////////////////Null Valur or OPTIONS Enum/////////////////
    let has_val1 = Some("Hello");
    let has_val2 = Some(55);
    let no_val: Option<i8> = None;

}

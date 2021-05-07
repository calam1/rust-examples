// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];
//
//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
//
//     println!("{:?}", v2)
// }

fn main() {
    // into_iter is a copy
    // iter is a reference/borrow

    // pass vector use iter().map with inline anonymous function
    let vals: Vec<i32> = vec![1, 2, 3, 4, 7];
    test_anonymous_function(&vals);

    // pass vector use into_iter().map with defined function
    let vals2: Vec<i32> = vec![1, 2, 3, 4, 7];
    test_defined_function(&vals2);

    // iter().map()
    let vals3: Vec<i32> = vec![1, 2, 3, 4, 7];
    let fnc0 = |x: &i32| -> i32 { *x + 1 };
    let test: Vec<_> = vals3.iter().map(fnc0).collect();
    println!("iter().map() {:?}", test);

    // into_iter().map()
    let vals4: Vec<i32> = vec![1, 2, 3, 4, 7];
    let fnc1 = |x: i32| -> i32 { x + 1 };
    let test1: Vec<_> = vals4.into_iter().map(fnc1).collect();
    println!("into_iter().map() {:?}", test1);


    // doesn't compile/work
    // test_wont_work();

}

// this won't compile
// fn test_wont_work<'a>() {
//     let vals4: Vec<i32> = vec![1, 2, 3, 4, 7];
//     let fnc2 = |x: &'a i32| -> Result<&'a i32, std::io::Error> {
//         if x % 2 == 0 {
//             Ok(x)
//         } else {
//             Err(std::io::Error::new(std::io::ErrorKind::Other, "hello"))
//         }
//     };
//     let test2: Vec<_> = vals4.iter().map(fnc2).collect();
//     println!("this won't work {:?}", test2);
// }

fn test_anonymous_function<'a>(vals: &'a Vec<i32>) {
    let fnc1 = |x: &'a i32| -> Result<&'a i32, std::io::Error> {
        if x % 2 == 0 {
            Ok(x)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "hello"))
        }
    };

    let test: Vec<_> = vals.iter().map(fnc1).collect();
    println!("pass vector use iter().map with inline anonymous function {:?}", test);
}

fn test_defined_function(vals: & Vec<i32>) {
    let test: Vec<_> = vals.iter().map(fnc2).collect();
    println!("pass vector use into_iter().map with defined function {:?}", test);
}

fn fnc2(x: &i32) -> Result<&i32, std::io::Error> {
    if x % 2 == 0 {
        Ok(x)
    }  else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "hello"))
    }
}
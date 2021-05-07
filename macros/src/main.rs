// macro_rules! add {
//     ($a:expr,$b:expr)=>{
//         {
//             $a+$b
//         }
//     }
// }

macro_rules! add_as {
    // ($a:expr,$b:expr,$typ:ty)=>{
    //     $a as $typ + $b as $typ
    // };

    // ($a:expr,$($b:tt)*)=>{
    //     {
    //         $a+add_as!($($b)*)
    //     }
    // };
    ($a:expr)=>{
        $a
    };
    ($a:expr,$($b:tt)*)=>{
       {
            $a+add_as!($($b)*)
       }
    }
}


macro_rules! add{
 // first arm in case of single argument and last remaining variable/number
    ($a:expr)=>{
        $a
    };
// second arm in case of two arument are passed and stop recursion in case of odd number ofarguments
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    };
// add the number and the result of remaining arguments
    ($a:expr,$($b:tt)*)=>{
       {
           $a+add!($($b)*)
       }
    }
}

fn main() {
    add!(1, 2);
    // println!("{}", add_as!(0,2,u8));
    println!("{}", add_as!(1,2,3,4));
    println!("{}", add!(1,2,3,4));
}

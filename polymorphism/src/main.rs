use rand::{thread_rng, Rng};


fn main() {
    // let roll: D6 = Die::roll();
    // println!("D6 {:?}", roll);

    // let roll: D8 = Die::roll();
    // println!("D8 {:?}", roll);

    let r: D6 = roll();
    println!("D6 {:?}", r);

    let r: D8 = roll();
    println!("D8 {:?}", r);

    println!("{:?}",  roll::<D6>());

    let escaped = try_dodge_attack(roll(), roll());
    println!(
        "{}",
        match escaped {
            true => "You dogded!",
            false => "Ouch! The attack hit you!",
        }
    );
}




// trait Roll<T> {
//     fn roll() -> T;
// }



fn try_dodge_attack(d6: D6, d8: D8) -> bool {
    d6.val() + d8.val() > 10
}




#[derive(Debug)]
struct D6(u8);

#[derive(Debug)]
struct D8(u8);

// struct Die {}

// impl Roll<D6> for Die {
//     fn roll() -> D6 {
//         D6 {
//             0: thread_rng().gen_range(1..=6)
//         }
//     }
// }

// impl Roll<D8> for Die {
//     fn roll() -> D8 {
//         D8 {
//             0: thread_rng().gen_range(1..=8)
//         }
//     }
// }

pub trait Rollable {
    fn roll() -> Self;
    fn val(&self) -> u8;
}

pub fn roll<T: Rollable>() -> T {
    Rollable::roll()
}

impl Rollable for D6 {
    fn roll() -> D6 {
        D6 {
            0: thread_rng().gen_range(1..=6),
        }
    }

    fn val(&self) -> u8 {
        self.0
    }
}

impl Rollable for D8 {
    fn roll() -> D8 {
        D8 {
            0: thread_rng().gen_range(1..=8),
        }
    }

    fn val(&self) -> u8 {
        self.0
    }
}

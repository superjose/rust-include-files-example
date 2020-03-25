// 'a is a lifetime annotation
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
pub struct Candy {
    category: CandyType,
    amount: i8,
}

impl Candy {
    pub fn new_chocolate(amount: i8) -> Candy {
        Candy {
            category: CandyType::Chocolate,
            amount,
        }
    }
    pub fn new_gummy_bear(amount: i8) -> Candy {
        {
            Candy {
                category: CandyType::GummyBear,
                amount,
            }
        }
    }
}

enum CandyType {
    Chocolate,
    GummyBear,
}

pub fn eat_dessert(candy: Candy) {
    match candy.category {
        CandyType::Chocolate => println!("Chocolate... Yum!! Eating {} chocolates", candy.amount),
        CandyType::GummyBear => println!("So Gummy... I'm eating {} gummy bears!", candy.amount),
    }

    println!("Chocolate.... Yum!!");
}

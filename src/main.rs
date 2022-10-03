fn main() {
    let e1 = Collection::set_result(1.0);

    let e2 = Collection::new(
        e1,
        Operator::Add,
        Collection::new(
            Collection::set_result(2.0),
            Operator::Add,
            Collection::set_result(3.0),
        ),
    );

    let mut e3 = Collection::new(e2, Operator::Mul, Collection::set_result(6.0));

    let mut temp = Collection::new(
        Collection::new(
            Collection::set_result(2.0),
            Operator::Add,
            Collection::set_result(3.0),
        ),
        Operator::Mul,
        Collection::set_result(6.0),
    );
    println!("temp: {}", temp.eval());
    println!("e3 is {:#?}", e3);
    println!("{}", e3.eval());
}
#[derive(Clone,Debug)]
struct Experssion {
    // left: Box<Collection>,
    right: Box<Collection>,
    opertor: Operator,
}
#[derive(Clone,Debug)]
enum Collection {
    Result(f64),
    Expression(Box<Collection>, Experssion),
    Nil,
}

impl Collection {
    fn new(left: Box<Collection>, opertor: Operator, right: Box<Collection>) -> Box<Self> {
        Box::new(Collection::Expression(left, Experssion { right, opertor }))
    }

    fn set_result(f: f64) -> Box<Self> {
        Box::new(Collection::Result(f))
    }

    fn set_order(&mut self) {
        /*
            from right we go to left
            we check the the right expression if it wasn't a raw result we put it in the left
            also we check the operator if it was a mul/div we put it in the left
        */

        while let Collection::Expression(left,right) =self{
            if let Collection::Expression(right_Left,right_Right ) = right{

            }
        }

    //     while let Collection::Expression(left, right) = &mut **self {
    //         if let Collection::Expression(left, right) = &mut **right.right {
    //             *self = Collection::Expression(
    //                 left.clone(),
    //                 Experssion {
    //                     right: right.right.clone(),
    //                     opertor: right.opertor.clone(),
    //                 },
    //             );
    //         } else {
    //             break;
    //         }
    //     }
    // }

    fn eval(&mut self) -> f64 {
        match self {
            Collection::Result(v) => *v,
            Collection::Expression(left, experssion) => {
                let left = left.eval();
                let right = experssion.right.eval();
                let result = match experssion.opertor {
                    Operator::Add => left + right,
                    Operator::Sub => left - right,
                    Operator::Mul => left * right,
                    Operator::Div => left / right,
                    Operator::None => left,
                };
                *self = Collection::Result(result);
                result
            }
            Collection::Nil => 0.0,
        }
    }
}

#[derive(Clone,Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    None,
}

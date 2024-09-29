
#[derive(Debug)]
enum Term<'a> {
    Value(f64),
    Add(&'a Term<'a>, &'a Term<'a>),
    Mul(&'a Term<'a>, &'a Term<'a>),
}

fn eval(expr: &Term) -> f64 {
    match expr {
        Term::Value(val) => *val,
        Term::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Term::Sub(lhs, rhs) => eval(lhs) - eval(rhs),
        Term::Mul(lhs, rhs) => eval(lhs) * eval(rhs),
        Term::Div(lhs, rhs) => eval(lhs) / eval(rhs),
    }
}

fn derive(expr: &Term, wrt: &Term) -> f64 {
    match expr {
        Term::Value(_) => if expr as *const _ == wrt as *const _ {
            1.
        } else {
            0.
        },
        // d(a + b)/dx = da/dx + db/dx
        Term::Add(lhs, rhs) => derive(lhs, wrt) + derive(rhs, wrt),
        // d(a * b)/dx = a * db/dx + da/dx * b
        Term::Mul(lhs, rhs) => eval(lhs) * derive(rhs, wrt) + derive(lhs, wrt) * eval(rhs),
    }
}

fn main() {
    let a = Term::Value(1.);
    let b = Term::Value(2.);
    let ab = Term::Add(&a, &b);
    let c = Term::Value(42.);
    let abc = Term::Mul(&ab, &c);
    println!("({a:?} + {b:?}) * {c:?} = {}", eval(&abc));
    println!("d(({a:?} + {b:?}) * {c:?})/dc = {}", derive(&abc, &c));
}

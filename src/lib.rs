

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse() {
//         assert_eq!(Number::new("123"),Number(123));
//     }
// }

// #[derive(Debug, PartialEq)]
#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number{
    pub fn new(s:&str)-> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Op{
    Add,
    Sub,
    Mul,
    Div,
}
impl Op{
    pub fn new(c:char)-> Self {
        match c {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            _ => panic!("Invalid operator"),
        }
    }
}
pub struct Expr{
    pub left: Number,
    pub op: Op,
    pub right: Number,  
}
impl Expr{
    pub fn new(s:&str)->self{
        let lhs=Number::new(&s[0..1]);
        let op=Op::new(s.chars().nth(1).unwrap());
        let rhs=Number::new(&s[2..3]);
        Self{lhs,op,rhs}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse(){
        assert_eq!(Number::new("123"),Number(123));
    }
    #[test]
    fn parse_nums() {
        assert_eq!(Number::new("123"),Number(123));
    }
    #[test]
    fn parse_add_op(){
        assert_eq!(Op::new('+'),Op::Add)
    }
    #[test]
    fn parse_sub_op(){
        assert_eq!(Op::new('-'),Op::Sub)
    }
    #[test]
    fn parse_mul_op(){
        assert_eq!(Op::new('*'),Op::Mul)
    }
    #[test]
    fn parse_div_op(){
        assert_eq!(Op::new('/'),Op::Div)
    }
    #[test]
    fn parse_one_plus_two(){
        let n1 = Number::new("1");
        let n2 = Number::new("2");
        let op = Op::new('+');
        assert_eq!(n1,Number(1));
        assert_eq!(n2,Number(2));
        assert_eq!(op,Op::Add);
    }
    #[test]
    fn parse_one_plus_two2()
}


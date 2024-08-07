
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse() {
//         assert_eq!(Number::new("123"),Number(123));
//     }
// }

// #[derive(Debug, PartialEq)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse(){
        assert_eq!(Number::new("123"),Number(123));
    }
}


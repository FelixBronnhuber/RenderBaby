mod scene;
mod geometric_object;
mod obj_parser;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::obj_parser::parseobj;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

    #[test]
    fn parse(){
        parseobj();
    }
}

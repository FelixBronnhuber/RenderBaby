pub struct Sum {
    l: u64,
    r: u64
}

impl Sum {
    pub fn new(l: u64, r: u64) -> Self {
        Sum{l, r}
    }

    pub fn compute(&self) -> u64 {
        computational_plane::add(self.r, self.l)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sum : Sum = Sum::new(1,3);
        let result = sum.compute();
        assert_eq!(result, 4);
    }
}

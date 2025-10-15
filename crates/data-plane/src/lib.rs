struct Sum {
    l: u64,
    r: u64
}

impl Sum {
    fn new(l: u64, r: u64) -> Self {
        Sum{l, r}
    }

    fn compute(&self) -> u64 {
        computational_plane::add(self.r, self.l)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

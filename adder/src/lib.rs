pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_worked() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn added(){
        panic!("Falied");
    }

}

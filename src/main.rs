fn main() {}

#[cfg(test)]
mod tests {
    use crdts::{CmRDT, MVReg, Map};

    #[test]
    fn pprint() {
        let mut r1 = MVReg::new();
        let mut r2 = r1.clone();
        let r1_read_ctx = r1.read();
        let r2_read_ctx = r2.read();

        r1.apply(r1.write("bob", r1_read_ctx.derive_add_ctx('a')));

        let op = r2.write("alice", r2_read_ctx.derive_add_ctx('b'));
        r2.apply(op.clone());

        r1.apply(op); // we replicate op to r1

        // Since "bob" and "alice" were added concurrently, we see both on read
        assert_eq!(r1.read().val, vec!["bob", "alice"]);
        assert_eq!(r2.read().val, vec!["alice"]);
    }

    #[test]
    fn map() {
        let mut m: Map<&str, MVReg<&str, _>, &str> = Map::new();

        println!("{:?}", m.len());
    }
}

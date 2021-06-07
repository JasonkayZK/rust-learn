pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
            pub fn another_nested_modules() {}
        }
    }
}

fn main() {
    use a::series::of;
    use a::series::of::another_nested_modules;

    of::nested_modules();
    another_nested_modules();
}

#[cfg(test)]
mod tests {

}

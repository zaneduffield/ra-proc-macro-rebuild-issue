fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[yare::parameterized(case = {0})]
    fn foo(_val: i32) {}
}

fn main() {
    cc::Build::new().file("unsafe/operations.c").compile("c_unsafe_functions");
}

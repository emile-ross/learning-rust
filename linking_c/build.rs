fn main() {
    cc::Build::new()
        .file("unsafe/operations.c")
        .file("unsafe/other/math.c")
        .compile("c_unsafe_functions");
}

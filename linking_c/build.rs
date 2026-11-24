fn main() {
    cc::Build::new()
        .file("unsafe/operations.c")
        .file("unsafe/other/math.c")
        .file("unsafe/alloc.c")
        .file("unsafe/input.c")
        .compile("c_unsafe_functions");
}

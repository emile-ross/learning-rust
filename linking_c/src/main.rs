fn main() {
    unsafe {
        hello_world();
    }

}

unsafe extern "C" {
    fn hello_world();  /* call */
}

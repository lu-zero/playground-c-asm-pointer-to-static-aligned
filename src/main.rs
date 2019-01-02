
#[repr(C, align(32))]
struct Aligned([u8; 16]);

const LEN: usize = 16;

#[no_mangle]
pub static L: usize = LEN;

static FOO: Aligned = Aligned([42; LEN]);

#[no_mangle]
pub static BAR: &[u8] = &FOO.0;

extern "C" {
    fn print_from_c();
}

fn main() {
    println!("From rust {:?} {:?}", BAR.as_ptr(), BAR);
    unsafe { print_from_c(); }
}

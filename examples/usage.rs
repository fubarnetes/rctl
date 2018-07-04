extern crate rctl;
extern crate libc;

fn main() {
    let uid = unsafe { libc::getuid() };

    let subject = rctl::Subject::user_id(uid);

    let usage = subject.usage().expect("Could not get RCTL usage");

    println!("{:#?}", usage);
}

/* automatically generated by rust-bindgen 0.69.4 */

extern "C" {
    pub fn rctl_get_racct(
        inbufp: *const ::std::os::raw::c_char,
        inbuflen: usize,
        outbufp: *mut ::std::os::raw::c_char,
        outbuflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rctl_get_rules(
        inbufp: *const ::std::os::raw::c_char,
        inbuflen: usize,
        outbufp: *mut ::std::os::raw::c_char,
        outbuflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rctl_get_limits(
        inbufp: *const ::std::os::raw::c_char,
        inbuflen: usize,
        outbufp: *mut ::std::os::raw::c_char,
        outbuflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rctl_add_rule(
        inbufp: *const ::std::os::raw::c_char,
        inbuflen: usize,
        outbufp: *mut ::std::os::raw::c_char,
        outbuflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rctl_remove_rule(
        inbufp: *const ::std::os::raw::c_char,
        inbuflen: usize,
        outbufp: *mut ::std::os::raw::c_char,
        outbuflen: usize,
    ) -> ::std::os::raw::c_int;
}

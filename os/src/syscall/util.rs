use core::mem::size_of;
use crate::task::current_user_token;

pub(crate) fn wirte_struct_to_vbuf<T>(src: T, dst: *mut T) -> isize {
    let mut curr = 0; let total = size_of::<T>();
    let buffers = crate::mm::translated_byte_buffer(current_user_token(), dst as *mut u8, total);
    unsafe {
        let tmp:&'static[u8] = core::slice::from_raw_parts(
            &src as *const T as *const u8, total
        );
        for buffer in buffers {
            let _end = buffer.len().min(total-curr) + curr;
            buffer.copy_from_slice(&tmp[curr.._end]);
            curr = _end;
        }
    }
    if curr == total {0} else {-1}
}

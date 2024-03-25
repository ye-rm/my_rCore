use core::time::Duration;

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}

fn endless() -> !{
    loop {}
}

pub fn sleep(hart_id:usize,sec: u64){
    use sbi_rt::{set_timer,hart_start,hart_stop};
    hart_start(hart_id,endless as usize, 0);
    set_timer(sec);
    hart_stop();
}

#![no_main]
#![no_std]

use aux8::*;

#[entry]
fn main() -> ! {
    let (gpioe, rcc, tim2, gpioa, tim6, mut dwt) = aux8::init();
    gpio::init(rcc, gpioa, gpioe);
    timer::init(rcc, tim6, 7_999, 100);
    pwm::init(tim2, 1000, 7);
    let mut input;
    let mut input_prev = false;
    let mut input_count = 0;
    let mut input_freq;
    let soll_freq = 500;
    let mut diff;
    
    loop {
        if timer::status_bit_is_set(tim6) {
            timer::restart(tim6);
            input_freq = input_count * 10;
            diff = soll_freq - input_freq;
            if input_freq > 1000 {
                pwm::set_duty_cycle(tim2, 0);
            } else {
                pwm::set_duty_cycle(tim2, (soll_freq + diff) as u16);
            }
            input_count = 0;
        }
        input = gpio::read_pin2(gpioa);
        
        if input && !input_prev {
            input_count += 1;
        }
        input_prev = input;
    }
}

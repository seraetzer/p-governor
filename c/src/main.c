#include "main.h"

#define soll_freq 500 // Setpoint

int main(void)
{
  init_gpio();
  init_timer();
  init_pwm();
  
  int input = 0;
  int input_prev = 0;
  int input_count = 0;
  int input_freq = 0; // Actual value
  int diff = 0;
  volatile unsigned int cycles = 0; // unused; only for monitoring purpose
  DWT->CTRL |= 1;

  while(1)
  {
    DWT->CYCCNT = 0; // Reset cycle counter
    if (tim6_status_bit_is_set())
    {
      tim6_restart();
      input_freq = input_count * 10;
      diff = soll_freq - input_freq;
      if (input_freq > 1000)
      {
        set_duty_cycle(0);
      }
      else
      {
        set_duty_cycle(soll_freq + diff);
      }
      input_count = 0;
    }

    input = read_pin2();

    if (input && !input_prev)
    {
      input_count++;
    }
    input_prev = input;
    cycles = DWT->CYCCNT; // Read cycle counter
  }
}
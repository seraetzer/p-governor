#include "main.h"

void init_timer(void)
{
  RCC->APB1ENR = RCC_APB1ENR_TIM2EN | RCC_APB1ENR_TIM6EN;
  TIM6->CR1 = 0;
  TIM6->CR2 = 0;
  TIM6->PSC = 7999;
  TIM6->ARR = 100;
  TIM6->CR1 = TIM_CR1_CEN | TIM_CR1_ARPE | TIM_CR1_OPM;
}

unsigned short tim6_status_bit_is_set(void)
{
    return TIM6->SR & TIM_SR_UIF;
}

void tim6_restart(void)
{
    TIM6->SR = 0;
    TIM6->CR1 |= TIM_CR1_CEN;
}

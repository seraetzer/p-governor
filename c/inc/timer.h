#ifndef __TIMER_H
#define __TIMER_H

void init_timer(void);
unsigned short tim6_status_bit_is_set(void);
void tim6_restart(void);

#endif
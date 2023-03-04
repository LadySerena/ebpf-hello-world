// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

// SEC MACRO takes this path $PROBE/SUBSYSTEM/THING
// sudo rg "syscalls" /sys/kernel/debug/tracing/available_events 
// ^ shows what's on the menu for tracing
SEC("tp/syscalls/sys_enter_clone")
int handle_clone(void *ctx){
  bpf_printk("HELLO WORLD\n");// generally not performant
  return 0;
}
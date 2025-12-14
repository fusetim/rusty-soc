MEMORY
{
   ram (rwx) : ORIGIN = 0x00000000, LENGTH = 64K
   stack (rwx) : ORIGIN = 0x0000f800, LENGTH = 0x800
   peripheral (rw) : ORIGIN = 0x00010000, LENGTH = 64K
}

REGION_ALIAS("REGION_TEXT", ram);
REGION_ALIAS("REGION_RODATA", ram);
REGION_ALIAS("REGION_DATA", ram);
REGION_ALIAS("REGION_BSS", ram);
REGION_ALIAS("REGION_HEAP", ram);
REGION_ALIAS("REGION_STACK", stack);

_stext = ORIGIN(REGION_TEXT);
_heap_size = 0;                                 /* Set heap size to 0KB */
_max_hart_id = 0;                               /* Single-core */
_hart_stack_size = 1K;                          /* Set stack size per hart to 1KB */
_stack_start = ORIGIN(stack) + LENGTH(stack);

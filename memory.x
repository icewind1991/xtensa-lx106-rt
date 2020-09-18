ENTRY(Reset)

/* Linker script for the ESP8266 */

MEMORY
{
      /* All .data/.bss/heap are in this segment. Reserve 1KB for old boot or ROM boot */
      dram_seg :     org = 0x3FFE8000, len = 0x14000

      vectors_seg :  org = 0x40100000, len = 0x100
      iram_seg :     org = 0x40100100, len = 0x8000 - 0x0100
      irom_seg :     org = 0x40220000, len = 0x3DA000
}

PROVIDE(__pre_init = DefaultPreInit);

PROVIDE(__user_exception = __default_exception);
PROVIDE(__kernel_exception = __default_exception);
PROVIDE(__double_exception = __default_double_exception);
PROVIDE(__nmi_exception = __default_exception);
PROVIDE(__debug_exception = __default_exception);
PROVIDE(__alloc_exception = __default_exception);
PROVIDE(__slc_interrupt = __default_interrupt);
PROVIDE(__spi_interrupt = __default_interrupt);
PROVIDE(__gpio_interrupt = __default_interrupt);
PROVIDE(__uart_interrupt = __default_interrupt);
PROVIDE(__ccompare_interrupt = __default_interrupt);
PROVIDE(__soft_interrupt = __default_interrupt);
PROVIDE(__wdt_interrupt = __default_interrupt);
PROVIDE(__timer1_interrupt = __default_interrupt);

PROVIDE(__naked_user_exception = __default_naked_user_exception);
PROVIDE(__naked_kernel_exception = __default_naked_kernel_exception);
PROVIDE(__naked_double_exception = __default_naked_double_exception);
PROVIDE(__naked_nmi_exception = __default_naked_nmi_exception);
PROVIDE(__naked_debug_exception = __default_naked_debug_exception);
PROVIDE(__naked_alloc_exception = __default_naked_alloc_exception);

/* needed to force inclusion of the vectors */
EXTERN(__default_exception);
EXTERN(__default_double_exception);
EXTERN(__default_interrupt);

EXTERN(__default_naked_user_exception);
EXTERN(__default_naked_exception);
EXTERN(__default_naked_double_exception);
EXTERN(__default_naked_nmi_exception);
EXTERN(__default_naked_debug_exception);
EXTERN(__default_naked_alloc_exception);

/* Define output sections */
SECTIONS {

  .vectors :
  {
    . = 0x0;
    _init_start = ABSOLUTE(.);
    . = 0x10;
    KEEP(*(.DebugException.text));
    . = 0x20;
    KEEP(*(.NMIException.text));
    . = 0x40;
    KEEP(*(.KernelException.text));
    . = 0x50;
    KEEP(*(.UserException.text));
    . = 0x70;
    KEEP(*(.DoubleException.text));
    . = 0x80;

    _init_end = ABSOLUTE(.);
  } > vectors_seg

  .rwtext :
  {
    *(.rwtext.literal .rwtext .rwtext.literal.* .rwtext.text.*)
  } > iram_seg

  .text :
    {
      _text_start = ABSOLUTE(.);
      *(.literal .text .literal.* .text.*)
      _text_end = ABSOLUTE(.);
    } > iram_seg

    .rotext :
    {
      *(.rotext.literal .rotext.text .rotext.literal.* .rotext.text.*)
    } > irom_seg

  /* Shared RAM */
  .dram0.bss (NOLOAD) :
  {
    . = ALIGN (8);
    _bss_start = ABSOLUTE(.);
    *(.bss)
    *(.bss.*)
    . = ALIGN (8);
    _bss_end = ABSOLUTE(.);
  } > dram_seg

  .dram0.data :
  {
    _data_start = ABSOLUTE(.);
    *(.data)
    *(.data.*)
    _data_end = ABSOLUTE(.);
  } >dram_seg

  _sidata = LOADADDR(.dram0.data);

  .dram0.rodata :
  {
    _rodata_start = ABSOLUTE(.);
    *(.rodata)
    *(.rodata.*)
    _rodata_end = ABSOLUTE(.);
    . = ALIGN(4);
    _heap_start = ABSOLUTE(.);
  } >dram_seg

}

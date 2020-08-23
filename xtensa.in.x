/* before memory.x to allow override */
ENTRY(Reset)

INCLUDE memory.x

/* after memory.x to allow override */
PROVIDE(__pre_init = DefaultPreInit);

PROVIDE(__exception = __default_exception);

/* Define output sections */
SECTIONS {

  .iram.text :
  {
    _stext = .;
    _text_start = ABSOLUTE(.);
    *(.literal .text .literal.* .text.*)
    _text_end = ABSOLUTE(.);
    _etext = .;
  } > iram_seg

  /* Shared RAM */
  .dram0.bss (NOLOAD) :
  {
    . = ALIGN (8);
    _bss_start = ABSOLUTE(.);
    *(.bss)
    *(.bss.*)
    . = ALIGN (8);
    _bss_end = ABSOLUTE(.);
  } >dram_seg

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

/* Proprietary ROM function needed for proper clock configuration.
 */
rom_i2c_writeReg = 0x400072d8;

PROVIDE ( _xtos_set_exception_handler = 0x40000454 );

/* Interrupt control ROM functions */
PROVIDE ( ets_isr_attach = 0x40000f88 );
PROVIDE ( ets_isr_mask = 0x40000f98 );
PROVIDE ( ets_isr_unmask = 0x40000fa8 );
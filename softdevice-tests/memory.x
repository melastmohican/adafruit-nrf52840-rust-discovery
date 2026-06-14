/*
 * Linker script for Adafruit Feather nRF52840 Express
 *
 * Flash layout (1 MB total):
 *   0x00000000 - 0x00000FFF   MBR          (4 KB)   — never touch
 *   0x00001000 - 0x00025FFF   SoftDevice S140 v6.x (~148 KB) — never touch
 *   0x00026000 - 0x000ECFFF   APPLICATION  (~796 KB) ← we live here
 *   0x000ED000 - 0x000F3FFF   User Data / LittleFS (28 KB)
 *   0x000F4000 - 0x000FFFFF   DFU Bootloader (48 KB)
 *
 * SRAM layout (256 KB total):
 *   0x20000000 - 0x200041FF   SoftDevice RAM (~16 KB, varies by SD version)
 *   0x20004200 - 0x2003FFFF   APPLICATION RAM (~240 KB) ← we live here
 *
 * Note: if you are NOT using BLE at runtime the SoftDevice still occupies
 * flash but does not claim any RAM, so you can use ORIGIN = 0x20000000
 * with LENGTH = 0x40000 for the RAM region in that case.
 */

/* S140 v6.1.1 (Adafruit default)
*MEMORY
*{
*   FLASH (rx)  : ORIGIN = 0x00026000, LENGTH = 0xCA000
*   RAM   (rwx) : ORIGIN = 0x20008000, LENGTH = 0x38000
}*/
/*S140 v7.2.0
*MEMORY
*{
*   FLASH (rx)  : ORIGIN = 0x00027000, LENGTH = 0xD9000
*   RAM   (rwx) : ORIGIN = 0x20020000, LENGTH = 0x20000
*}
*/

/*S140 v7.3.0*/
MEMORY
{
    FLASH (rx)  : ORIGIN = 0x00027000, LENGTH = 0xD9000
    RAM   (rwx) : ORIGIN = 0x20005a08, LENGTH = 0x3A5F8
}

# ESP32-S3 ULP Experiment

This experiment demonstrates the interaction between the Xtensa main cores and the RiscV ULP core.
There are two seperate programs one for the Xtensa core and one for the RiscV core. Both have a seperate build process and linked to together into one executable. First the esp32-ulp-blink (RiscV)needs to be compiled and than the esp-ulp-test (Xtensa).
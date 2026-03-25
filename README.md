# parallelresist
<a href="https://github.com/msilveus/parallelresist">Parallel resistor calculator</a>

Written in Rust

Usage: parallelresist.exe --target-resistance <TARGET_RESISTANCE> --target-wattage <TARGET_WATTAGE> --individual-wattage <INDIVIDUAL_WATTAGE> --file-path <FILE_PATH>

Usage: parallelresist.exe -r <TARGET_RESISTANCE> -w <TARGET_WATTAGE> -i <INDIVIDUAL_WATTAGE> -f <FILE_PATH>

Example:

$ parallelresist.exe  -r 1234 -w 3 -i 1 -f inventory.txt 

Target: 1234Ω (0.0008103727714748784S), Min Resistors: 3 
Checking combinations of exactly 3 resistors...          
Checking combinations of exactly 4 resistors...          
Checking combinations of exactly 5 resistors...          
Checking combinations of exactly 6 resistors...          
Checking combinations of exactly 7 resistors...          
Checking combinations of exactly 8 resistors...          
Checking combinations of exactly 9 resistors...          
Checking combinations of exactly 10 resistors...         
Perfect or near-perfect match found.                     
                                                         
--- Final Result ---                                     
Resistance: 1234.000685 Ω                                
Error:      0.000685 Ω (0.0001%)                         
  x1  4700 Ω                                             
  x2  6800 Ω                                             
  x2  10000 Ω                                            
  x1  15000 Ω                                            
  x1  47000 Ω                                            
  x1  100000 Ω                                           
  x1  220000 Ω                                           
  x1  1000000 Ω                                          
                                                         

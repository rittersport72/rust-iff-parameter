# rust-iff-parameter 
The Distributed Interactive Simulation (DIS) protocol transmits Identification Friend or Foe (IFF) in a 
Packet Data Unit (PDU). This library helps with encoding and decoding of IFF mode parameters.  
In IEEE DIS 1278.2-2015 annex B the different encodings for IFF Mode 1, 2, 3A, C, 4, 5, S are described.

## Mode 3A Code
Mode 3A Code is used in civil aviation as transponder code for aircraft identification.  
Mode 3A record shall be included in Parameter 3 of Fundamental Operational Data record of IFF PDU.
This is a 16 bit record. The code range is from 0000 to 7777 octal.  
For example for Mode 3A code 7461:  
The first digit (7) is code element (A), next digit (4) is code element (B), next digit (6) is code element (C), and last digit (1) is code element (D).

| Field name         | Bits    | Value        |
|--------------------|---------|--------------|
| Code element 1 (D) | 0 to 2  | 0 to 7 octal |
| Code element 2 (C) | 3 to 5  | 0 to 7 octal |
| Code element 3 (B) | 6 to 8  | 0 to 7 octal |
| Code element 4 (A) | 9 to 11 | 0 to 7 octal |
| Padding            | 12      | 0            |
| On/Off status      | 13      | Enumeration  |
| Damage status      | 14      | Enumeration  |
| Malfunction status | 15      | Enumeration  |

## Usage

```rust
// Create M3A record
let mut m3a = M3aRecord::default();

// M3A struct encoding
m3a.set_code(OctalCode(7, 4, 6, 1));
m3a.set_on_off(true);
m3a.set_damage(true);
m3a.set_malfunction(true);

// M3A struct decoding
let code = m3a.get_code();
let on_off = m3a.get_on_off();
let damage = m3a.get_damage();
let malfunction = m3a.get_malfunction();

// Get data as u16
let data = m3a.get();
// Set data from u16
m3a.set(0b0100_0101_0101_0101);
```

# References
Distributed Interactive Simulation https://en.wikipedia.org/wiki/Distributed_Interactive_Simulation  
IEEE Standard for DIS 1278.2-2015 https://standards.ieee.org/ieee/1278.2/6202

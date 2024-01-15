# rust-iff-parameter 
The Distributed Interactive Simulation (DIS) protocol transmits Identification Friend or Foe (IFF) in a 
Packet Data Unit (PDU). This library helps with encoding and decoding of IFF mode parameters.  
In IEEE DIS 1278.2-2015 annex B the different encodings for IFF Mode 1, 2, 3A, C, 4, 5, S are described.

## Mode 3A Code
Mode 3A Code is used in civil aviation as transponder code for aircraft identification.  
Mode 3A record is included in Parameter 3 of Fundamental Operational Data record of IFF PDU.
This is a 16 bits record. The code range is from 0000 to 7777 octal.  
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

### Usage

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

## Mode 5 Basic Data
Mode 5 is used in NATO military aviation as transponder code for aircraft identification.  
Mode 5 Basic Data record is included in the Mode 5 transponder format for layer 3 of the IFF PDU. This is a 128 bits record.

| Field name                     | Bits |
|--------------------------------|------|
| Mode 5 Status                  | 16   |
| Personal Identification Number | 16   |
| Mode 5 Message Format Present  | 32   |
| Enhanced Mode 1                | 16   |
| National Origin                | 16   |
| Supplemental data              | 8    |
| Navigation Source              | 8    |
| Figure of Merit                | 8    |
| Padding                        | 8    |

### Usage

```rust
// Create M5 record
let mut m5 = M5Record::default();

// M5 struct encoding
m5.set_pin(42);
m5.set_enhanced_mode_1(26);
m5.set_national_origin(711);

// M5 struct decoding
let pin = m5.get_pin()
let em1 = m5.get_enhanced_mode_1();
let no = m5.get_national_origin();

// Convert struct to byte stream
let array = m5.to_bytes();

// New message
let mut object = M5Record::default();

// Convert byte stream to struct
object.from_bytes(&array);
```

## Mode C Altitude
Mode C Altitude is used in civil aviation to transmit encoded altitude information.  
Mode C record is included in Parameter 5 of Fundamental Operational Data record of IFF PDU.
This is a 16 bits record.

| Field name         | Bits    | Value            |
|--------------------|---------|------------------|
| Altitude Indicator | 0       | Enumeration      |
| Mode C Altitude    | 1 to 11 | Unsigned Integer |
| Padding            | 12      | 0                |
| On/Off status      | 13      | Enumeration      |
| Damage status      | 14      | Enumeration      |
| Malfunction status | 15      | Enumeration      |

### Usage

```rust
// Create MC record
let mut mc = MCRecord::default();

// MC struct encoding
mc.set_altitude_msl(true);
mc.set_altitude(42);

// MC struct decoding
let msl = mc.get_altitude_msl();
let altitude = mc.get_altitude();

// Get data as u16
let data = mc.get();
// Set data from u16
mc.set(0b0100_0101_0101_0101);
```

# References
Distributed Interactive Simulation https://en.wikipedia.org/wiki/Distributed_Interactive_Simulation  
IEEE Standard for DIS 1278.2-2015 https://standards.ieee.org/ieee/1278.2/6202
 
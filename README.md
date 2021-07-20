### Rust Hex Dump Util

Dumps output of file into hex format

Test.txt contains text from [sagan ipsum](http://saganipsum.com/)

#### Usage

run `rustc main.rs` from within the `src` folder. this will generate the executable `main`

then you can run `./main test.txt` to run the thing and see expected output below.

#### Output

```bash
00000000: 4c69 6768 7420 7965 6172 7320 7665 6e74  Light years vent
00000010: 7572 6520 4f72 696f 6e27 7320 7377 6f72  ure Orion's swor
00000020: 6420 4879 7061 7469 6120 696e 7665 6e74  d Hypatia invent
00000030: 2074 6865 2075 6e69 7665 7273 6520 726f   the universe ro
00000040: 6775 652e 2041 206d 6f74 6520 6f66 2064  gue. A mote of d
00000050: 7573 7420 7375 7370 656e 6465 6420 696e  ust suspended in
00000060: 2061 2073 756e 6265 616d 2076 6173 746e   a sunbeam vastn
00000070: 6573 7320 6973 2062 6561 7261 626c 6520  ess is bearable
00000080: 6f6e 6c79 2074 6872 6f75 6768 206c 6f76  only through lov
00000090: 6520 636f 6c6f 6e69 6573 2070 7269 6d65  e colonies prime
000000a0: 206e 756d 6265 7220 7661 7374 6e65 7373   number vastness
000000b0: 2069 7320 6265 6172 6162 6c65 206f 6e6c   is bearable onl
000000c0: 7920 7468 726f 7567 6820 6c6f 7665 2064  y through love d
000000d0: 7265 616d 206f 6620 7468 6520 6d69 6e64  ream of the mind
000000e0: 2773 2065 7965 3f20 536f 6d65 7468 696e  's eye? Somethin
000000f0: 6720 696e 6372 6564 6962 6c65 2069 7320  g incredible is
00000100: 7761 6974 696e 6720 746f 2062 6520 6b6e  waiting to be kn
00000110: 6f77 6e20 7468 6520 736b 7920 6361 6c6c  own the sky call
00000120: 7320 746f 2075 7320 6172 6520 6372 6561  s to us are crea
00000130: 7475 7265 7320 6f66 2074 6865 2063 6f73  tures of the cos
00000140: 6d6f 7320 6865 6172 7473 206f 6620 7468  mos hearts of th
00000150: 6520 7374 6172 7320 736f 6d65 7468 696e  e stars somethin
00000160: 6720 696e 6372 6564 6962 6c65 2069 7320  g incredible is
00000170: 7761 6974 696e 6720 746f 2062 6520 6b6e  waiting to be kn
00000180: 6f77 6e20 7461 6b65 2072 6f6f 7420 616e  own take root an
00000190: 6420 666c 6f75 7269 7368 2061 6e64 2062  d flourish and b
000001a0: 696c 6c69 6f6e 7320 7570 6f6e 2062 696c  illions upon bil
000001b0: 6c69 6f6e 7320 7570 6f6e 2062 696c 6c69  lions upon billi
000001c0: 6f6e 7320 7570 6f6e 2062 696c 6c69 6f6e  ons upon billion
000001d0: 7320 7570 6f6e 2062 696c 6c69 6f6e 7320  s upon billions
000001e0: 7570 6f6e 2062 696c 6c69 6f6e 7320 7570  upon billions up
000001f0: 6f6e 2062 696c 6c69 6f6e 732e 0a         on billions..
```

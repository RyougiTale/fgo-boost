## fgo-boost

---

**description**
A fgo ios bluetooth helper.
Get inspiration from McLaren12345.
You can find details by clicking below link.
[FGO_Bluetooth_Assistant](https://github.com/McLaren12345/FGO_Bluetooth_Assistant)

---

**how to use**
I will make a release version some day later.
It maybe only support Windows10 and iphone8plus.
I'll make adaptation for ipad and other diffenert mobiles.

---

**how to build**
You can build it by yourself.
But it's a little complex cause of I use c++ lib opencv and I use c++ to write windows gdi capturing.(Maybe I will write it by rust native code in future)
1. go to [opencv-rust](https://github.com/twistedfall/opencv-rust) to set Environment variables.
2. use your Visual Studio to compile c++ code file to dynamic library.(I will add a makefile in future)
3. add dynamic library you need to path for linking.]
4. cargo build

---

**why use rust**
just try a new language
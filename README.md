# keeboard

###Note for FORGE Reviewers:
I had already bought the parts for this keyboard. I'm looking for reimbursment, not funding, if that is possible (I beg of thee it is).
I'm sorry that the prices for everything is so high. If you review the BOM, nothing would really be buildable without all the parts I bought. I tried to find the cheapest parts, but I realized that shipping minimum is $8, so then ordering from multiple 3rd party websites, prices would stack up considerably. So then I chose to order from Amazon. Please don't be taken aback by the price, I tried to make it as cheap as possible. The total price was just under $300. 
Please understand why I spent so much. First of all, Trump. Ordering the PCB from JLCPCB costed $45 or so dollars for the bare pcb. Then shipping costs came in, and then also tarrifs. that more than doubled the price to $100, but with the in ap $20 discount I reduced it to $80.
But all these microbumps and macrobumps in price really increased how much I had to spend. Please understand me. I had viewed the budget to be like $180, but it was really concerning how much the price started increasing by. 
PLEASE PLEASE approve this review. I beg of thee. Of courser, changes might need to be made, and I'm fine with that. But on the long term I don't want to lose the money I invested in this project, I did ordered prematurely because Intially I was doing projects via Stardance, but I realized NO WAY i could make deadline. So I swapped to this :) also I know double dipping will get me banned. Don't even tempt me.  I fully swapped over to this.

BY THE WAY, I HAVE ALL THE RECIEPTS AND EVERYTHING SAVED!!! PM me on slack: raghavsharma1214 if you have any inquiries about why price is so high. or just anything.

I'll delete this rant once approved :)

## Overview

This is an ANSI104 layout keyboard. Meaning 100% size, full numpad, and everything. It has creamy gateron milky yellow pro v2 switches, and an esp32 controlling it all. Finally, it is a gasket mount to dampen vibrations, and a silicon pour for the same effect, and to add more weight.  

I created this keyboard because I wasn't really satisfied with the keyboards on the market, and the current keyboard I have now. I want to follow the aesthetic I want, and not some drab boring tech-grey color or whatever, while keeping my own physical case style as well. 

One unique thing i'm proud of is crafting my own Gasket mount system. Instead of using clunky o-ring systems, I designed a brand new board-wide poron tape design. Like foam, it works like a sandwitch above and below the keyboard plate, and designed to provide maximum creamieness.

Editable CAD file: https://drive.google.com/file/d/1waTcEkKPX5QRJGWGog04dOq1EIFDCUFI/view?usp=sharing
Uploading to drive due to the file been 500mb, too large to upload to github.

## Gallery:
<img width="2560" height="1311" alt="2" src="https://github.com/user-attachments/assets/e5b2c563-a7fc-4e6d-9a71-581b715fa542" />
<img width="1412" height="449" alt="Screenshot 2026-09-18 212607" src="https://github.com/user-attachments/assets/c1f07fe7-93d2-4d5c-9608-f066a385f381" />
<img width="2213" height="920" alt="Screenshot 2026-09-18 212728" src="https://github.com/user-attachments/assets/fbd95075-ac13-46cc-8097-b3dfbccccc9a" />


## Future Improvements

Optimize the keyboard case design, maybe don't have as many bulky screws. Make the firmware better I guess, minimize latency so that I can game on it. Of course, the keyboard isn't physically built at the moment. 
---

## ASSEMBLY
### ** haven't built yet. Waiting for approval by FORGE **
1. Solder on the diodes and kailh hotswap sockets
2. Solder in the esp32
3. Plug in the switches, with the plate in the middle
4. place PORON foam in the gasket mounts sandwitch style
5. place the plate in between the gaskets
6. place heatset inserts in bottom and screw in m3 screws via the top
7. flash firmware
8. Pour in the silicon pour underneath the PCB, make sure to seal it off with seran wrap first.
9. enjoy!

# Bill of Materials (BOM)

### Electronics & Hardware

| Item | Description | Cost (Pre‑Tax) | Link |
| :--- | :--- | :--- | :--- |
| Switches | Gateron Milky Yellow Pro V2 | $25.90 | https://www.amazon.com/gp/product/B0C2CZJQHT |
| Rotary Encoder | ALPS EC11 THT 20k thread-blue vertical switch (H: 20mm) | $5.58 | https://mouser.com |
| Keycaps | PBT green, MOA profile | $24.99 | https://www.amazon.com/gp/product/B0F914Q43R |
| Diodes | 1N4148 SMD (Qty: 104), SOD‑123 | $0.18/unit | https://www.mouser.com/en/ProductDetail/Diodes-Incorporated/1N4148W-7-F |
| Stabilizers | Durock V3 screw‑in | $25.99 | https://www.amazon.com/dp/B0B2RW12S2 |
| PCB | Custom PCB | $45 | AIVON (with $30 discount applied) |
| Kailh Hotswap Sockets | 120 pcs | $13.20 | https://mechanicalkeyboards.com/products/kailh-switch-hot-swap-socket |
| ESP32‑S3 DevKitC‑1 (N16R8) | MCU | $8.99 | https://www.amazon.com/dp/B0GBT212KM |
| Cyanoacrylate Glue | CA super glue | $5.99 | https://www.amazon.com/dp/B0DYJPW9GK |

### Fasteners & Case Hardware

| Item | Description | Cost | Link |
| :--- | :--- | :--- | :--- |
| M3 Heatset Inserts | Brass | $6.99 | https://www.amazon.com/dp/B0FWWW8VP1 |
| M3 Screws | Assorted set | $6.99 | https://www.amazon.com/dp/B0D3X5CT2J |
| 3D Printed Case | Custom | — | — |
| 3D Printed Plate | Custom | — | — |
| 3D Printed Knob | EC11 compatible | — | — |
| Adhesive Transfer Tape | 3M Scotch 924 ATG | $9.57 | https://www.amazon.com/dp/B00FFDUG9M |

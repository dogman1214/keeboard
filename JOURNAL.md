---
title: "KEEBoard"
author: "Raghav Sharma"
description: "A custom mechanical keyboard designed for maximum thoccieness"
created_at: "2026-07-18"
---
# 7/18/2026: Setting up and Schematics:
This entry incloudes the setup fo the github, planning parts, size of the keyboard, and starting the schmatics in KiCad.

The schematic required a lot of copy and pasting, since it required ~100 combinations of keys and diodes and all of that sort of stuff.
<img width="1735" height="967" alt="image" src="https://github.com/user-attachments/assets/7ba1e03b-a515-42bd-a62b-9693f6fefd7f" />
I had to learn that we needed diodes, which specific ones (1n4148), keyboard layout sizes (100%, 60% etc..) and whatnot. Basically made myself familiar with everything in this project ,and also made the schematics.
<img width="1767" height="760" alt="image" src="https://github.com/user-attachments/assets/63a12ab9-a12e-4c74-9592-b7b5afb4fbef" />
<img width="1671" height="837" alt="image" src="https://github.com/user-attachments/assets/2d1f5f19-7ff0-48eb-a331-521f61f6b76e" />
did annotating schematic and all, assigned footprints and all (made sure the switches are kailh hotswap socket)
Next steps would be the pcb design. Will get really messy lmao.

**Total time spent: 4.2 hours**


# 7/19/2026: PCB took a lot of time
Setting up the PCB took hella time. I also changed the schematic. The layout i used to have was very cursed and had a semi-numpad so I changed it to have a normal numpad or something like that. SO now its like an 104 key thing layout. Has a baby numpad and also a numpad.
I also had to change the motherboard thing to a Teensy 4.1, then changed to an esp32-S3 because the teensy doesn't have a USB-C port, because I dont have enough gpio pins to accomodate all the keys (just 1 row short), even when using matrix wiring<img width="1162" height="607" alt="image" src="https://github.com/user-attachments/assets/16a1c16a-2578-4c77-9b3a-f06051267643" />
<img width="634" height="579" alt="image" src="https://github.com/user-attachments/assets/be7ad50d-af1e-4831-b0f2-3ae5d5b86276" />
The problem is that the PCB's keys are all jumbled up. I have to arrange the keys, in a specific order according to matrix wiring, locate which key to put where, space it properly with stabilizers, etc etc. It was SO SO SO DAMN Annoying!!!!
<img width="943" height="792" alt="image" src="https://github.com/user-attachments/assets/6115a56f-0861-4d28-ba0a-c458e49a1253" />
I spent so long doing all this that I got kind of pissed off its legit 2:14 AM rn
<img width="1033" height="1194" alt="image" src="https://github.com/user-attachments/assets/07107123-3787-4331-9642-d4a56aafb0a6" />
<img width="1033" height="1194" alt="image" src="https://github.com/user-attachments/assets/240a1a7d-89b6-4d14-9fdc-2dc498d38469" />

**Total time spent: 8 hours**

# 7/21/2026: Editing schematics
Today I started off by finishing the schamtic. Assigned all the columns rows and thingamajigs to the ESP32 S3, and ddi the footprint assigning. (REALLY REALLY ANNOYING SINCE I FOUND OUT YOU HAVE TO SPECIFY WHICH TYPE OF SWITH< THE SIZE OF THE SWITCH & THE SIZE OF THE STABIIZER AND ALL.)
<img width="1773" height="850" alt="image" src="https://github.com/user-attachments/assets/385365b0-f919-4365-b843-15bd52fe4a02" />
Then I realized THE ESP32 S3 WONT EVEN BLOODY WORK! IT DOESNT HAVE ENOUGH GPIO PINS THAT WORK> APPARENTLY I CAN"T USE THE GPIO PINS 0, 3, 45, 46 WHICH IS BLOODY WIERD WTF DUDE.

BUT WAIT! I failed to account for the BLoody DIODES!!!!!!! HAHAHAHHhahahaHAHAHAHAHA THE DIODES MAKE IT WORK IVE BEEN RUNNING IN CIRCLES FOR NOTHING LMAOOOOOOOOOOOOOOOOOOOOOOOOO

I added more stabilizers btw for what I need (7 in total).

The footprint assigning I used AI to help me know what keys in the ANSI 104 layooutneeded big stabs or big keys.
<img width="967" height="949" alt="image" src="https://github.com/user-attachments/assets/3f7eaf5b-ea45-43e3-9781-23a3f801c9c7" />
big jumpble.

---- LATER ON ---
In the schematic I had used the generic esp32-s3 schematic, just the chip itself, so there wasn't a footprint for it. So I had to replace it with the esp32-s3 devkit c1, which had the actual board, and had to re=assign all the connections to that. took a decent amount of time since there were so many wires to route. But I now fixed all the footprints and all and am transitioning to pcb design.
<img width="667" height="733" alt="image" src="https://github.com/user-attachments/assets/9f2f5d7b-baea-4dc8-bbe7-b23990ec37da" />
Edit: I ran ERC and realized I had placed the labels wrong. I placed the labels above the wires, when I was supposed to place it's square ON the wire so it registers its connected to it. Now I gotta move all the labels. >:( I also have to trim the wiring and all to fit. I lowk did a bit sloppy so making it more neat. I also have to add the no-connection flags and all.
<img width="703" height="856" alt="image" src="https://github.com/user-attachments/assets/8e41c4de-d335-4e2d-b345-3ef33ef82079" />
<img width="1764" height="817" alt="image" src="https://github.com/user-attachments/assets/b458f3f0-09a4-4334-946e-bade289b523e" />
BTw now getting on with some Deeeelicious PCB stuff

**Total time spent: 4 hours**


# 7/22/2026: PCB stuff hell yeah!
I setting up pcb and all, ordering in the locations and stuff; it's taking quite a while but i'm doing it. I made some schematic changes to fix DRC and ERC but nothing major. I just found out the esp32-s3-Devkit-c has 2 usb-c ports which is kind of dumb lmao. Idk whether I should keep in the render
<img width="358" height="478" alt="image" src="https://github.com/user-attachments/assets/37073664-0895-4a19-bd61-6cc3484dad7a" />
yeah just a lot of moving going on rn

**Total time spent: 3.4 hours**

# 7/22/2026: Working on PCB lines
Right now, I'm experiencing a few conflicts in the design stuff. first off all, there are multiple versions of the 1n418 diode. one is THT other is on the wall or whatever (w). THT diodes are massive, so I wanna use the tiny 1n4148w diodes instead will be much better. Yes, better yes. Used the D-sod-123 diode footprint.
<img width="144" height="160" alt="image" src="https://github.com/user-attachments/assets/66fc7210-039d-4bd6-929e-214c92272373" />
So now I set it as that, we have to go onto th wiring thingamajigs.

But i realized, the diodes are in the wrong location. I am wiring them to pin two, not one, so now I have to move all the diodes to their respective locations.

I was wondering, How would I go about wiring the PCB routes and make the wires not intersect, since they are in a matrix type of wiring. But then I realized, I acn use the top of the pcb for the rows and bottom for the columns, or vice versa.

I flipped all the kailh hotswap ports onto the back.

I also made a kinda du mb mistake, i routed the tracks to the soder points which were on the oppsoite side of the keyboard lmao.
<img width="979" height="729" alt="image" src="https://github.com/user-attachments/assets/24b23809-a14a-4c3e-b8c5-ec99882a43fe" />
Huh... something wierd happened wiht my key placement & wiring
<img width="511" height="1045" alt="image" src="https://github.com/user-attachments/assets/58643066-7d8c-4a97-b40e-7de91987f6c6" />
I have an extra key idk how did this happen but the keyboard layout is normal. Things are funked out what?

nvm it was the esc key i had forgotten.

I finished the column wiring..
<img width="1405" height="580" alt="image" src="https://github.com/user-attachments/assets/c2b9764a-58fd-4e02-a196-2dacb2d00420" />

**Total time spent: 4.03 hours**

# 7/26/2026: Row wirint etc...
Now I'm doing the row wiring... Also I forgot a diode on the esc key so I'm adding that.

there was sme funky thing about obstacle avoidance, so i had to manyally avoid those obstacles.
<img width="1012" height="784" alt="image" src="https://github.com/user-attachments/assets/c70a18ba-f671-421c-8621-844b947e6bbf" />

**Total time spent: 1.5 hours**

# 7/31/2026: wiring n stuff
The wiring to the actual gpio pins is getting hella hella cursed like I cannot describe how much.
<img width="591" height="814" alt="image" src="https://github.com/user-attachments/assets/14fdfc9c-7a81-4e24-87df-c8dba55d33d8" />
the layout of the footprint vs in the schematic is far different, so the wires are getting all jumbled. I'm also wiring with alternating directions kind of to make this a tad bit simpler.

Once I was drebugging this connection where I was missing a column but I had just wired 2 gpio pins to one column it was so dumb lmao

Next gunna start validation of pcb.
I legit took an hour tryig to fit column wires to the gpio pins cuz the gpio pins are not LABELED IN FLIPPING order!!! So its like trying to navigate a maze.
<img width="402" height="772" alt="image" src="https://github.com/user-attachments/assets/a4f01056-1c65-44fd-848b-39e4d11e91ec" />

**Total time spent: 2.5 hours**

# 8/1/2026: I just realized...
I js realised that I didn't wire up the diodes to the key. I gotta do that for every single damn key.
<img width="463" height="448" alt="image" src="https://github.com/user-attachments/assets/b49d909b-5bc2-4735-9a26-56fca45f1f4f" />
<img width="694" height="607" alt="image" src="https://github.com/user-attachments/assets/aad0f1f7-37a4-4e3c-89e5-a52d614c5068" />
Now I gotta wire the rotary encoder.

And then, I realized again that I have an extra column, so I have to assign that to a spare gpio port. A bit messy. I wired it to gpio 16. The only problem is that I have two paths now, fix all the rows and rewire evverything, or accept it'll get a bit messy and then make the code a bit , a tiny bit unintuitive. I'm going with the second one.

So this is how the overall pcb looks like rn:
<img width="1201" height="436" alt="image" src="https://github.com/user-attachments/assets/f0f7fdbb-235f-470b-8847-15b0179cd8ea" />
then, ii realized that the edges of the pcb are too wide. so I'm trimming it down to 7mm thick edges, while also adjusting the wires as well for it.
<img width="1231" height="451" alt="image" src="https://github.com/user-attachments/assets/1b5afc59-3214-4a6c-a75b-e40eda508b90" />
So now that I double chekced the tracks and all, I think now its time for the case design!

**Total time spent: 3 hours**

# 8/1/2026: it is NOT time for case design.
I'm mopping more things up on the pcb; one major problem was how close some wires/routes are to each other. It gets a bit sketchy, so I'm making it a bit more roomy. I don't know what are the error bounds of the pcb manufacturer I'll order from, soI would rather be safe then sorry. Then I manually inspect eveyrrthing double triple diple check
<img width="658" height="757" alt="image" src="https://github.com/user-attachments/assets/bf01c487-80ca-40f9-871a-2e34db790b7e" />
<img width="409" height="879" alt="image" src="https://github.com/user-attachments/assets/668b5687-6fae-47e7-a120-9c89cb1bcb99" />
this section was hella close to each other, so I kind of adjusted it a bit for it to be a bit more roomy.

**Total time spent: 0.4 hours**


# 8/7/2026: Starting the case fr
I'll start the case design in freeCAD, then transition to blender for the final stuff. Kicad is good for like measurements n stuff, but I'm used to blender so I think I should finish up with that with final touches and cosmetic additions.

OOOÓ MYYYYY PCCCCCC
<img width="1857" height="685" alt="image" src="https://github.com/user-attachments/assets/295f9fd2-0dda-43e8-a183-6038808b32fd" />
THE BLOODY STEP FILE INCORPORATGED EVERY SINGLE MOLECULE AS AN INDIVIDUAL PART

ITS LAGGING MY PCCC

wait what
<img width="1305" height="172" alt="image" src="https://github.com/user-attachments/assets/c3369efe-b95c-4763-aa2a-a1b586d897ff" />
why is edge taking all my resources wtf its not freecad

so I hid all the tiny solderparts and kept the green pcb thingy visible for now only.

so what I want to do is that not make the case 3d printed, but like that is probably the way to go with a translucent look.
<img width="1033" height="529" alt="image" src="https://github.com/user-attachments/assets/c9b6135a-21e6-47f5-8202-fc1b509d4296" />
Okay. so starting out with modeling, cuz this is a gasket mount, it is different then the usual 3d design. I want to add foam padidng underneath the keyboard to make it sound thoccy..

The whole design should follow this analogy or something idk:
<img width="891" height="493" alt="image" src="https://github.com/user-attachments/assets/479a26ba-8d35-4ff7-991c-649957789faa" />
<img width="480" height="289" alt="image" src="https://github.com/user-attachments/assets/6c8b55ac-7d7a-4728-9726-4d4e6ff6cb2c" />
(just use one piece of foam no need for a billion layers though)

I like this kind of aeshtetic for the colros:
<img width="1057" height="544" alt="image" src="https://github.com/user-attachments/assets/b8122881-c307-4212-8c12-25f1f9af720b" />
or maybe this:
<img width="1066" height="553" alt="image" src="https://github.com/user-attachments/assets/afed3dab-9109-40ac-abdb-be116b3010b1" />
wait though. I forgot to assign 3d models to each footprint in kicad. Sigh

Ugh. Anyway, I'm planning possible I could order the keyboard case to be CNC'ed out of wood. that would be fun and thoccy.

**Total time spent: 4 hours**


# 8/8/2026: Making more case stuff
Okay so now I legit started making the case and all. Yeah it is going to be wood CNC'ed for extra thoccyness.

ooh yeah we gettin somehwere
<img width="1513" height="784" alt="image" src="https://github.com/user-attachments/assets/6d947a4d-5fb8-4c60-b5b2-0d83a01f0149" />
<img width="2968" height="1486" alt="image" src="https://github.com/user-attachments/assets/adf61c52-5ddc-4d95-a66b-023e67b5f6d2" />
<img width="2437" height="1569" alt="image" src="https://github.com/user-attachments/assets/65266040-a540-4ec2-b8d8-5dacd27c49f5" />
<img width="2571" height="1644" alt="image" src="https://github.com/user-attachments/assets/dacd8b65-c0f0-4a59-b388-16afea3ae7b2" />
<img width="2382" height="1327" alt="image" src="https://github.com/user-attachments/assets/1cb0b8f0-73f7-4193-a622-e2cdf772b699" />
So I chatted with a dude on slack about this whole timing thing, and he said that it is better to do timelapses, so taht's probably what I will do.

Yeah so I logged just 40 min of ts.

But like, doing wood cnc is unlikly cuz it costs way too much. I'm confused.

**Total time spent: 2 hours**


# 8/14/2026: Importing from macondo
Transitioning from Macondo
I realized that I won’t be able to complete this project in time for macondo, so, instead I will do the project here!
All the logs for the creation process are on macondo, I can provide as per request, but I’ll be continuing off of here.
<img width="241" height="900" alt="image" src="https://github.com/user-attachments/assets/1783acca-4aee-4378-9e80-e785365703a6" />

**Total time spent: 0.1 hours**

# 8/14/2026: PCB design
Changes, and advancements in pcb design…
I decided that i will use a top-mount case, much easier to manufacture than gasket mount, which I am alien to… kind of.
What I’m doing is kind of a combination of top mount case and gasket mount. I will use a foam tape as a sort of gasket type thing, but it will span the entire perimeter of the keyboard, instead of specific gasket areas, allowing me to get a proper thocky sound but at an easier way to design it.
I’m also placing all the keys now for visual representation.
<img width="1401" height="900" alt="image" src="https://github.com/user-attachments/assets/e1ca6191-89ba-4cfa-a092-14d426e7492f" />

**Total time spent: 1.45 hours**

# 8/15/2026: Finished placing keyboards
Finished placing the keys, a daunting task.
I had to manually place the keys, reference what the offset is compared to otehrso n the guide, etc etc.
And get this: It is all just for the visual cad represenation when this finishes.
lmao
<img width="1600" height="714" alt="image" src="https://github.com/user-attachments/assets/0b08f3cd-0697-43c2-a4d0-38d843fc24dd" />

**Total time spent: 1.5 hours**

# 8/15/2026: Going to blender
Now I’m transitioning to blender for the case design. I like blender because I can fine tune the aesthetics and all.
SO I set up the case basics and everything, like the outline of the case, dividing into two parts. Very roguht right now. it looks pretty nice. I used a pre-designed set of keycaps online because I won’t be making those, but ordering them instead. anyway, it all looks pretty nice!

<img width="1414" height="900" alt="image" src="https://github.com/user-attachments/assets/b86f0079-af38-47d4-87b2-36a9bba9b384" />

**Total time spent: 1.6 hours**

#8/15/2026: Making the case look nicer
Working on the keyboard case…
I made the exterior look all nice (ish), but I didn’t create the semi-gasket mount. An invention of my own
The “gasket” is just foam tape going around the whole perimeter of the keeb’s plate. Absorbing vibrations and enhancing “thoccieness”
Hell yeah
I still have to finish the case. It isn’t close to being done yet. This is a rough “sketch”

<img width="1600" height="864" alt="image" src="https://github.com/user-attachments/assets/2c34e5eb-05b9-4a6c-b17a-26ebd4ae774a" />

**Total time spent: 2.8 hours**

# 8/16/2026: redesigned the bloody case
Okay so I swapped up the case design agian. I had to redesign eveyrthing because the top part of the case is too shallow for the esp-32 s3.
Also I have to make sure the walls have thickness right now they are just 0mm thick.
BTW, what i’m doing in this keyboard case, is something rarely done before. I’m creatign a gasket mount but no the regular kind. THis one will consist of poron foam tape spanning the entire perimeter as the “gasket”
<img width="1600" height="861" alt="image" src="https://github.com/user-attachments/assets/72e331ad-2aa3-43f3-970b-3f462a346294" />

**Total time spent: 2.15 hours**


# 8/16/2026: making more parts of the keyboard case
building even more of the case
I finished up the general shape of the case. I created an opening for the USB-C port, and now I have to give the walls actual thickness and stuff, and check for stuff. This case design is really cursed because instead of combining two walls, I just placed the verticies in the same place instead of joining lmao
<img width="1584" height="900" alt="image" src="https://github.com/user-attachments/assets/c9123a3e-62fe-45cf-9975-d8ac593ae4a2" />

**Total time spent: 1.5 hours**

# 8/16/2026: stukkyy wukkky
My design is cooked…
The plate is same in dimensions as the PCB, so there is no way for the gasket support to fit between PCB and plate without losing structural integrity or just not fitting if I’m 3d printing. So I think the workaround here is to widen the plate across the y-axis and use those protrusions as support for the gaskets.
Idk I’ll see how the flow goes lowk.
Even though right now it is a kind-of gasket mount, this classifies it as a semi kind-of gasket mount lmao
Also, I’m deciding that I will do a silicon pour under the PCB for maximum sound insulation. it will also add weight to the keyboard, to keep it anchored and NOT rattly.
next will be to double check all parts of keyboard, make sure everything has proper thickness, and hope for the best!
<img width="1600" height="714" alt="image" src="https://github.com/user-attachments/assets/9917691a-330f-4e6b-9148-25377d94d799" />

**Total time spent: 1.5 hours**

# 8/18/2026: Hopefully finalized design.
I finalized the keyboard design (hopefully…)
I added sections for rubber feet
Made the case slightly longer for the bottom to have some thickness, and all sorts of other stuff. Now onto rendering or sum idk.
<img width="1600" height="625" alt="image" src="https://github.com/user-attachments/assets/75a000a0-df94-4271-96b2-51f1fb1ac668" />

**Total time spent: 1.2 hours**

# 8/18/2026: Rendering
Doing the rendering. Changing the keyboard colors and everythign and making them look nice. pretty annoying, tried to color mach from keychron website.
<img width="1600" height="576" alt="image" src="https://github.com/user-attachments/assets/e664e290-79b1-4668-9a31-7e39ed0e04c7" />

**Total time spent: 0.5 hours**

# 8/19/2026: working more on rendering
Places a lot of objects and set up scene + lighting. changed keyboard colors a bit, working on camera placement and everything
<img width="895" height="554" alt="image" src="https://github.com/user-attachments/assets/3503d097-7d7c-4fc0-905a-a7569d0eb2ff" />

**Total time spent: 1.55 hours**

# 8/21/2026: Completed final render
Did all the final rendering stuff, set up a custom camera with custom lens simulations and everything. had to render mulltiple time si had these weird white lines coming through whihc i had to solve (just deleted them...)
<img width="1600" height="819" alt="image" src="https://github.com/user-attachments/assets/38f122d9-4cdf-4e6c-ae07-1ad5e54ea36a" />

**Total time spent: 3.2 hours**

# 8/23/2026: Working on the parts and everything
I forgot to put screw holes i just remembered. will havve to do that but i was researching what parts to use and everything (like gateron milky yellow v2 for switches) and everything. 
<img width="1142" height="900" alt="image" src="https://github.com/user-attachments/assets/ee2fe73e-93a4-429d-ac03-162ed6bb0343" />

**Total time spent: 1.7 hours**

# 8/23/2026: Fixing some things and adding hole designs.
Blender kept freezing though. Maybe i'll do it tomorrow if my laptop has a good day. But i'm attempting to create holes for screws and everything.
<img width="1600" height="771" alt="image" src="https://github.com/user-attachments/assets/784413fd-9e62-48de-9801-0b4fd4895f1b" />

**Total time spent: 0.7 hours**

# 8/27/2026: Able to work on holes actually
I removed the bazillion verticies that were frying laptop. Used remesh mod for that, but topology got messed up. Its fine thouhg, 3d printing will smoothen it out. Bit im a bit conflicted about the holes.
<img width="1600" height="873" alt="image" src="https://github.com/user-attachments/assets/b101bec7-5cc3-4c92-91db-9d24a9fe1f39" />

**Total time spent: 1.7 hours**

#8/30/2026: Finishing the screws
I had to add independent screw holes and everytihg. pretty annoying since i had to like use boolean cut on a billion cylinders then anotehr subset of cylinders then more blah blah blah. I didn't like this parto ne bit.
<img width="1384" height="689" alt="image" src="https://github.com/user-attachments/assets/7ee474f9-c0ec-43c2-8e25-e69991754eaf" />

**Total time spent: 1.5 hours**

# 9/7/2026: Big gap, bu did conversion and mesh combinging.
Polishing the keyboard.
I converted the STLs to .STEP (a very annoying process btw), but before that i had to optimize the whole thing. I had to combine meshes and everything because all the files were totaling ~1g, and frying my RAM. Also I continued creating the BOM, and validating the PCB and stuff. This was SOO slow. because my ram was maxed out, I had to use SWAP. then blender, i had to wait like ~15 minutes between each operation because it ran hella slow. But I started researching different things like which keycaps I should use, which pour, and whatnot. BUt the whole optimization was so freaking tedious it took my whole bloody afternoon
<img width="676" height="120" alt="image" src="https://github.com/user-attachments/assets/41ddfbaa-96a9-46c0-84ff-776baf58947b" />

**Total time spent: 6 hours**

# 9/10/2026: Working on the BOM and github
Working on the keyboard and BOM.
I decided to remove small ledges underneed the keyboard case for the rubber stoppers. I can just glue them on underneath. Having those ledges just gives the chance of messing up the 3d print. So I’ll do it like that. Also, I’m working on the github. I am solidifying the whoel readme and BOM
<img width="1269" height="900" alt="image" src="https://github.com/user-attachments/assets/7b05f2e6-2cfa-4ba0-93b9-7b9cdb70dfd5" />

**Total time spent: 2.1 hours**

# 9/13/2026: Major redesign
righty guys so i changed some PCB things, sorted out through budget (was pushing $300 but now down to 220) and I split the prints in half so that they can actually be printed. I had to create brand new screw holes and patch existing holes and stuff with cursed geometry, like i had to add more screw holes to attach both side togehter, because i realized i couldn't print a whole 500mm keyboard case at once, so thats pretty cursed.
<img width="932" height="861" alt="image" src="https://github.com/user-attachments/assets/dcf5f834-4a2e-41fb-96c4-b57fbea82ba2" />

**Total time spent: 6.2 hours**

# 9/14/2026: Gunna order parts
I just wrapped up some things, teaked the case design (removed some wierd ass bezels and stuff) and then chopped the plate into one more piece so i can print it laying flat down. because the case is oging to be printed vertical, but not teh plate.
<img width="932" height="861" alt="image" src="https://github.com/user-attachments/assets/ba4eaa1d-ef50-434d-83a2-8bb273cda3cb" />

**Total time spent: 0.1 hours**


# 9/24/2026: Soldering parts
I have now finished soldering the Kailh Sockets. This was a genuine pain in the bum. First, I had to experiment soldering on another PCB, since I recieved 5 in total, and then found out a strategy to solder the sockets. First, I soldered the left pad by itself, then aligned the socket and heated up that pad to join the socket. Then I soldered the empty socket when its jammed in place, so nothing moves. Wierdly effective, but time consuming. I had to do everything one by one. Place on ething, solder the pad, solder the first socket, solder other side, rinse and repeat. One. Hundred. and Four times. Genuinely took forever smh. I'm genuinely concerned about how I'll do the teensy weensy tiny SMD diodes since I only have a blunt soldering tip and a thick soldering wire with rosin. 
<img width="1241" height="1655" alt="20260922_175707" src="https://github.com/user-attachments/assets/6dcf8684-be52-4c0c-b868-99e428825aca" />
<img width="1230" height="1639" alt="20260924_164847" src="https://github.com/user-attachments/assets/b98c6db0-99c3-4bde-adea-a0533c328409" />
**Total time spent: 5 hours**





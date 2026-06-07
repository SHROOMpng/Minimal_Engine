<h1>MINIMAL ENGINE</h1>

<h3>

- [**HOW TO USE IN A FEW STEPS**](#how-to-use-in-a-few-steps)
<small>
- [**CREATE A MAP AND IMPORT IT**](#create-a-map-and-import-it)
- [CREATE A TEXTURE AND IMPORT IT](#create-a-texture-and-import-it)
- [HOW TO EXPLORE YOUR VIRTUAL ENVIRONMENT](#how-to-explore-your-virtual-environment)

</small>

&nbsp;

- [**HOW IT WORKS**](#how-it-works)
<small>
- [INPUT FOLDERS](#input-folders)
- [Maps Folder](#maps-folder)
- [Textures Folder](#textures-folder)
- [SCRIPT](#script)
- [How to Update Script](#how-to-update-script)
- [Texture List Guide](#textures-list-guide)
- [Functioning 1: Which Map is Loaded](#functioning-1-which-map-is-loaded)
- [Functioning 2: Which Textures are Loaded](#functioning-2-which-textures-are-loaded)
- [DIRECTORY TREE](#directory-tree)

</small>
</h3>


&nbsp;
# **HOW TO USE IN A FEW STEPS**

## CREATE A MAP AND IMPORT IT:
The map file is in .png. <br>
Every pixel represent a tile in the virtual environment you are creating. There isn't any limit in the size.

Start by creating a fully black .png, and only add one white pixel where you want the player to spawn. Don't add more than one white pixel, because the engine needs this information to position the player. <br>
Black pixels represent walls where the player cant go, so you'll need to draw in with another color around the white tile (where the player spawn) to upsize the explorable environment. <br>
If you didn't add your own textures yet, you will need to use the pre-made/"placeholder" ones first. Use for example "100, 100, 100" as RGB values (grey) to make "concrete" corridors. <br>
If it doesn't work, make sure the "concrete" texture folder is in the "textures folder" *(please refer to "Textures Folder" in "INPUT FOLDERS")*.

To import your new map/play it in the engine, you will first need to import it in the "maps" folder, with the name you want *(please refer to "Maps Folder" in "INPUT FOLDERS")*. <br>
You will then need to replace the map file path in the script (ln 588) with the name you chose for your map. Don't forget the file format at the end (.png), and the file path at the start such as this example: "../maps/example_map.png"
*(please refer to "Functioning 1: Which Map is Loaded" in "SCRIPT" for more info)*.


&nbsp;
## CREATE A TEXTURE AND IMPORT IT:
Textures are stored in .png. <br>
There is a texture for every ceiling, floor and wall that can be rendered on the player screen *(please refer to "Texture folder" in "INPUT FOLDERS", and "Functioning 2: Which Textures are Loaded" in "SCRIPT" for more info)*. Each ceiling, floor and wall texture is in 4 variants (facing_x, facing_-x, facing_y, facing_-y), one for each orientation that the player can take.

To make a texture, start by creating a file with the same architecture as any other texture *(please refer to the "example_texture" folder in the "DIRECTORY TREE")* or copy-paste it from another. You can then start to draw anything on textures. Try and experiment as you wish! <br>
For information, textures are loaded in this order: background -> floor -> ceiling -> walls from left to right. The last texture being loaded will be the one layering on top of the others. By keeping in mind this order, you can add details that layer on each others to be coherent with the perspective. <br>
Also note that the tiles in front of the player are loaded in the following order, considering that the player is at (x, y) facing -y: (x-1, y-1) -> (x+1, y-1) -> (x-1, y) -> (x+1, y) -> (x, y-1) -> (x, y). <br>
In case you want to radically stop using textures (such as the ceiling to make an open sky environment), don't delete it. You will actually need to make it fully transparent, otherwise the engine will keep trying to load a non-existent file.

Once the folder for your new texture is finsished, you can add it to the textures folder *(please refer to "Texture folder" in "INPUT FOLDERS")*. You will now need to pair it to a RGB value so that you can apply it to tiles/pixels on your map. To do so, *please refer to the "Textures List Guide" in "SCRIPT"*. Make sure that the name you've putted in the Texture List in the script is exactly the same as the one you've chosen for the folder.


&nbsp;
## HOW TO EXPLORE YOUR VIRTUAL ENVIRONMENT:
To try your creation, your textures/map needs to be imported in first. For that, *refer to the last paragraphs in "CREATE A MAP AND IMPORT IT" and "CREATE A TEXTURE AND IMPORT IT"*.

You then need to launch the script. This part heavily depends on its version. The "indev_v013" version isn't compiled in its final form, due to the fast evolution in the script developement. <br>
To launch it uncompiled, you need to run it through the cargo commands:
- Open a CMD Shell
- Change the repo to the one where is installed the script, command example:
```"cd C:\User\Desktop\minimal_indev_v013"```
- Run the cargo, command example: ```cargo run```

&nbsp;
# **HOW IT WORKS**

## INPUT FOLDERS:

### Maps Folder

**"maps"** is the folder where the maps are stored in .png. Each pixel represents a tile where the player can go. 
- **Black** ones are walls where the player can't go
- **White** one is the player spawn 
- **All the others colors** are explorable tiles

Each color used as explorable tiles in the map needs their RGB values to be added to the texture list. <br>
<small>*(please refer to the "Textures List Guide" in "SCRIPT")*</small>

&nbsp;
### Textures Folder

**"textures"** is the folder where textures are stored. Each texture directory you want to use needs to be paired to a RGB value in the texture list, to then be rendered when the loaded map .png contains the same RGB value. <br>
<small>*(please refer to the "Textures List Guide" in "SCRIPT")*</small>

Textures are organized in 4 variants (facing_x, facing_-x, facing_y, facing_-y), one for each direction that the player can take. Each variant contains textures for the ceiling, the floor and the walls in front of the players.



&nbsp;
## SCRIPT:

**".../minimal_indev_v..." is the script folder.**

### How to Update Script

To update, drag the new script folder in the same directory *(please refer to the "DIRECTORY TREE" underneath)*. Multiple versions can be stacked, however, not all the versions will work the same way. Please refer to the documentation of each version.

&nbsp;
### Textures List Guide

The textures list is presented like this in the script... <br>
<small>.../minimal_indev_v.../src/main.rs, ln 21:</small>

```
fn get_material(r: u8, g: u8, b: u8) -> &'static str {
    match (r, g, b) {
		(255, 255, 255) => "example1_texture_directory",
        _ => "debug1", // fallback
    }
}
```

**"(255, 255, 255)" ->** white pixel, where the player spawn. <br>
**"example1_texture_directory" ->** name of the texture folder in "textures" which is loaded for this tile. <br>
*(please refer to the "DIRECTORY TREE" underneath, and the "Texture Folder" in "INPUT FOLDERS" for more info on the textures variants)*

To add a new texture named **"example2_texture_directory"** on red pixels (**"200, 0, 0"** in RGB values), write: <br>
<small>.../minimal_indev_v.../src/main.rs, ln 21:</small>

```
fn get_material(r: u8, g: u8, b: u8) -> &'static str {
    match (r, g, b) {
		(255, 255, 255) => "example1_texture_directory",
        (200, 0, 0) => "example2_texture_directory",
        _ => "debug1", // fallback
    }
}
```

&nbsp;
### Functioning 1: Which Map is Loaded

The script loads the map specified at this line... <br>
<small>.../minimal_indev_v.../src/main.rs, ln 588:</small>

```
let loaded_map = ImageReader::open("../maps/example1_map.png")
```

Here the script loads "example1_map.png" in the maps folder. <br>
<small>*(please refer to the "DIRECTORY TREE" underneath, and the "Map Folder" in "INPUT FOLDERS")*</small>

To load "example2_map.png" in the "maps" folder, write: <br>
<small>.../minimal_indev_v.../src/main.rs, ln 588:</small>

```
let loaded_map = ImageReader::open("../maps/example2_map.png")
```

&nbsp;
### Functioning 2: Which Textures are Loaded

"V" represents the rendered tile where the player is ; "." represents the other rendered tiles ; and "#" represents the render limit:
```
#####
#...#
#.V.#
#####
```
The walls are rendered only if the tile is considered as an unexplorable wall (represented as a black pixel (0, 0, 0 in rgb) in the map). <br>
<small>*(please refer to "Functioning 1: Which Map is Loaded" in "SCRIPT" for more info on how the map is loaded)*</small>

The textures are loaded in their "facing_x", "facing_-x", "facing_y" or "facing_-y" variant according to the player orientation. <br>
<small>*(please refer to the "DIRECTORY TREE" underneath, and the "Texture Folder" in "INPUT FOLDERS" for more info on the textures variants)*</small>

Note that the tiles in front of the player are loaded in the following order, considering that the player is at (x, y) facing -y: (x-1, y-1) -> (x+1, y-1) -> (x-1, y) -> (x+1, y) -> (x, y-1) -> (x, y):
```
#####
#152#
#364#
#####
```
The textures of each tile are then loaded in the following order: background -> floor -> ceiling -> walls from left to right.




&nbsp;
## DIRECTORY TREE:

The folders need to be organized this way:
```
├─ textures
│  └─ example_texture
│     ├─ facing_x
│     │  ├─ ceiling_x+1y0.png
│     │  ├─ ceiling_x+1y-1.png
│     │  ├─ ceiling_x0y0.png
│     │  ├─ ceiling_x0y-1.png
│     │  ├─ ceiling_x-1y0.png
│     │  ├─ ceiling_x-1y-1.png
│     │  ├─ floor_x+1y0.png
│     │  ├─ floor_x+1y-1.png
│     │  ├─ floor_x0y0.png
│     │  ├─ floor_x0y-1.png
│     │  ├─ floor_x-1y0.png
│     │  ├─ floor_x-1y-1.png
│     │  ├─ Fwall_x+1y0.png
│     │  ├─ Fwall_x+1y-1.png
│     │  ├─ Fwall_x0y0.png
│     │  ├─ Fwall_x0y-1.png
│     │  ├─ Fwall_x-1y0.png
│     │  ├─ Fwall_x-1y-1.png
│     │  ├─ Lwall_x0y0.png
│     │  ├─ Lwall_x0y-1.png
│     │  ├─ Lwall_x-1y-1.png
│     │  ├─ Rwall_x+1y-1.png
│     │  ├─ Rwall_x0y0.png
│     │  └─ Rwall_x0y-1.png
│     ├─ facing_-x
│     │  └─ ... (each facing has its textures in the same architecture as "facing_x")
│     ├─ facing_y
│     │  └─ ... 
│     └─ facing_-y
│        └─ ...
├─ minimal_indev_v...
│  ├─ msvc
│  │  ├─ dll
│  │  │  └─ 64
│  │  │     ├─ SDL2.dll
│  │  │     ├─ SDL2_image.dll
│  │  │     └─ SDL2_ttf.dll
│  │  └─ lib
│  │     └─ 64
│  │        ├─ SDL2.lib
│  │        ├─ SDL2_image.lib
│  │        └─ SDL2_ttf.lib
│  └─ src
│     ├─ main.rs
│     └─ texture_manager.rs
└─ maps
   └─ example_map.png

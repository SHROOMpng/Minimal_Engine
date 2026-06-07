use image::ImageReader;

use numpy::ndarray::Array3;
use ndarray::s;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::rect::{Rect};

use std::time::Duration;
use std::path::Path;



// FONCTIONS DÉFINITION DES TUILES

// Définitions variables des décors (personnalisables)
fn get_material(r: u8, g: u8, b: u8) -> &'static str {
    match (r, g, b) {
        (200, 100, 100) => "debug2",
        (255, 255, 255) => "street1_blank",
        (100, 100, 100) => "concrete",
        (200, 200, 100) => "corridor_blank",
        (250, 200, 100) => "corridor_furniture",
        (250, 250, 100) => "corridor_window",
        (170, 200, 100) => "corridor_x_door",
        (150, 150, 200) => "street1_blank",
        (170, 150, 200) => "street1_junk",
        (120, 150, 200) => "street1_-x_door",
        (150, 120, 200) => "street2_blank",
        (170, 120, 200) => "street2_garbage",
        _ => "debug1", // fallback
    }
}

// Définitions invariables (à ne pas manipuler)
fn get_pixel(map: &Array3<u8>, x: usize, y: usize) -> (u8, u8, u8) {
    let pixel = map.slice(s![y, x, ..]);
    (pixel[0], pixel[1], pixel[2])
}

fn is_empty_tile(r: u8, g: u8, b: u8) -> bool {
    r == 0 && g == 0 && b == 0
}

fn is_not_empty_tile(r: u8, g: u8, b: u8) -> bool {
    r > 0 && g > 0 && b > 0
}

fn is_spawn_tile(r: u8, g: u8, b: u8) -> bool {
    r == 255 && g == 255 && b == 255
}



// FONCTION DE RENDU

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

pub mod texture_manager;

fn render(canvas: &mut WindowCanvas, texture_manager: &mut texture_manager::TextureManager<WindowContext>, _texture_creator: &TextureCreator<WindowContext>,  _font: &sdl2::ttf::Font, loaded_map_array: &Array3<u8>, player_pos: Option<(usize, usize, i32)>, map_width: u32, map_height: u32) -> Result<(), String> {
    canvas.clear();
    
    // DRAW TEXTURES

    let src = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let dest = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    if let Some((x, y, facing)) = player_pos {

        // DÉFINITION DU PATH DES TEXTURES EN FONCTION DU facing DE player_pos(x, y, facing)

        let facing_str = match facing {
            0 => "facing_-y",
            1 => "facing_x",
            2 => "facing_y",
            3 => "facing_-x",
            _ => "facing_-y",
        };

        // Background
        let (pr, pg, pb) = get_pixel(loaded_map_array, x, y);
        let material_background = get_material(pr, pg, pb);

        let background_path = format!("../textures/{}/{}/background.png", material_background, facing_str);
        let background_texture = texture_manager.load(&background_path)?; 
        canvas.copy(&background_texture, src, dest)?;
        

        
        // CASE player_pos(x-1, y-1)

        // pixel en face à gauche du joueur en diagonale (player_pos(x-1, y-1))
        let (diag_l_x, diag_l_y) = match facing {
            0 => (x as i32 - 1, y as i32 - 1),
            1 => (x as i32 + 1, y as i32 - 1),
            2 => (x as i32 + 1, y as i32 + 1),
            3 => (x as i32 - 1, y as i32 + 1),
            _ => (x as i32, y as i32),
        };

        if diag_l_x >= 0 && diag_l_y >= 0 && diag_l_x < map_width as i32 && diag_l_y < map_height as i32 {

            let (lr, lg, lb) = get_pixel(loaded_map_array, diag_l_x as usize, diag_l_y as usize);
            let material_diag_l = get_material(lr, lg, lb);

            // Déterminer le matériau
            let floor_path = format!("../textures/{}/{}/floor_x-1y-1.png", material_diag_l, facing_str);
            let ceiling_path = format!("../textures/{}/{}/ceiling_x-1y-1.png", material_diag_l, facing_str);
            let front_path = format!("../textures/{}/{}/Fwall_x-1y-1.png", material_diag_l, facing_str);
            let left_path = format!("../textures/{}/{}/Lwall_x-1y-1.png", material_diag_l, facing_str);

            let tex_floor = texture_manager.load(&floor_path)?;
            let tex_ceiling = texture_manager.load(&ceiling_path)?;
            let tex_front = texture_manager.load(&front_path)?;
            let tex_left = texture_manager.load(&left_path)?;

            // Affichage du sol et du plafond
            if is_not_empty_tile(lr, lg, lb) {
                canvas.copy(&tex_ceiling, src, dest)?;
                canvas.copy(&tex_floor, src, dest)?;
            }

            // Affichage (ou non) du mur d'EN FACE en fonction de player_pos
            let (front_x, front_y) = match facing {
                0 => (x as i32 - 1, y as i32 - 2),
                1 => (x as i32 + 2, y as i32 - 1),
                2 => (x as i32 + 1, y as i32 + 2),
                3 => (x as i32 - 2, y as i32 + 1),
                _ => (x as i32, y as i32),
            };

            if front_x >= 0 && front_y >= 0 && front_x < map_width as i32 && front_y < map_height as i32 {
                let (r, g, b) = get_pixel(loaded_map_array, front_x as usize, front_y as usize);
                if is_empty_tile(r, g, b) {
                    canvas.copy(&tex_front, src, dest)?;
                }
            }

            // Affichage (ou non) du mur de GAUCHE en fonction de player_pos
            let (left_x, left_y) = match facing {
                0 => (x as i32 - 2, y as i32 - 1),
                1 => (x as i32 + 1, y as i32 - 2),
                2 => (x as i32 + 2, y as i32 + 1),
                3 => (x as i32 - 1, y as i32 + 2),
                _ => (x as i32, y as i32),
            };

            if left_x >= 0 && left_y >= 0 && left_x < map_width as i32 && left_y < map_height as i32 {
               let (r, g, b) = get_pixel(loaded_map_array, left_x as usize, left_y as usize);
                if is_empty_tile(r, g, b) {
                    canvas.copy(&tex_left, src, dest)?;
                }
            }
        }



        // CASE player_pos(x+1, y-1)

        // pixel en face à droite du joueur en diagonale (player_pos(x+1, y-1))
        let (diag_r_x, diag_r_y) = match facing {
            0 => (x as i32 + 1, y as i32 - 1),
            1 => (x as i32 + 1, y as i32 + 1),
            2 => (x as i32 - 1, y as i32 + 1),
            3 => (x as i32 - 1, y as i32 - 1),
            _ => (x as i32, y as i32),
        };

        if diag_r_x >= 0 && diag_r_y >= 0 && diag_r_x < map_width as i32 && diag_r_y < map_height as i32 {

            let (dr, dg, db) = get_pixel(loaded_map_array, diag_r_x as usize, diag_r_y as usize);
            let material_diag_r = get_material(dr, dg, db);

            // Déterminer le matériau
            let floor_path = format!("../textures/{}/{}/floor_x+1y-1.png", material_diag_r, facing_str);
            let ceiling_path = format!("../textures/{}/{}/ceiling_x+1y-1.png", material_diag_r, facing_str);
            let front_path = format!("../textures/{}/{}/Fwall_x+1y-1.png", material_diag_r, facing_str);
            let right_path = format!("../textures/{}/{}/Rwall_x+1y-1.png", material_diag_r, facing_str);

            let tex_floor = texture_manager.load(&floor_path)?;
            let tex_ceiling = texture_manager.load(&ceiling_path)?;
            let tex_front = texture_manager.load(&front_path)?;
            let tex_right = texture_manager.load(&right_path)?;

            // Affichage du sol et du plafond
            if is_not_empty_tile(dr, dg, db) {
                canvas.copy(&tex_ceiling, src, dest)?;
                canvas.copy(&tex_floor, src, dest)?;
            }

            // Affichage (ou non) du mur d'EN FACE en fonction de player_pos
            let (front_x, front_y) = match facing {
                0 => (x as i32 + 1, y as i32 - 2),
                1 => (x as i32 + 2, y as i32 + 1),
                2 => (x as i32 - 1, y as i32 + 2),
                3 => (x as i32 - 2, y as i32 - 1),
                _ => (x as i32, y as i32),
            };

            if front_x >= 0 && front_y >= 0 && front_x < map_width as i32 && front_y < map_height as i32 {
                let (r, g, b) = get_pixel(loaded_map_array, front_x as usize, front_y as usize);
                if is_empty_tile(r, g, b) {
                    canvas.copy(&tex_front, src, dest)?;
                }
            }

            // Affichage (ou non) du mur de DROITE en fonction de player_pos
            let (right_x, right_y) = match facing {
                0 => (x as i32 + 2, y as i32 - 1),
                1 => (x as i32 + 1, y as i32 + 2),
                2 => (x as i32 - 2, y as i32 + 1),
                3 => (x as i32 - 1, y as i32 - 2),
                _ => (x as i32, y as i32),
            };

            if right_x >= 0 && right_y >= 0 && right_x < map_width as i32 && right_y < map_height as i32 {
                let (r, g, b) = get_pixel(loaded_map_array, right_x as usize, right_y as usize);
                if is_empty_tile(r, g, b) {
                    canvas.copy(&tex_right, src, dest)?;
                }
            }
        }



        // CASE player_pos(x-1, y)

        // pixel à gauche du joueur (player_pos(x-1, y))
        let (left_x, left_y) = match facing {
            0 => (x as i32 - 1, y as i32),
            1 => (x as i32, y as i32 - 1),
            2 => (x as i32 + 1, y as i32),
            3 => (x as i32, y as i32 + 1),
            _ => (x as i32, y as i32),
        };

        if left_x >= 0 && left_y >= 0 && left_x < map_width as i32 && left_y < map_height as i32 {

            let (lr, lg, lb) = get_pixel(
                loaded_map_array,
                left_x as usize,
                left_y as usize,
            );

            let material_left = get_material(lr, lg, lb);

            // Déterminer le matériau
            let floor_path_l = format!("../textures/{}/{}/floor_x-1y0.png", material_left, facing_str);
            let ceiling_path_l = format!("../textures/{}/{}/ceiling_x-1y0.png", material_left, facing_str);
            let front_path_l = format!("../textures/{}/{}/Fwall_x-1y0.png", material_left, facing_str);

            let texture_floor_l = texture_manager.load(&floor_path_l)?;
            let texture_ceiling_l = texture_manager.load(&ceiling_path_l)?;
            let texture_front_l = texture_manager.load(&front_path_l)?;

            // Affichage du sol et du plafond
            if is_not_empty_tile(lr, lg, lb) {
                canvas.copy(&texture_ceiling_l, src, dest)?;
                canvas.copy(&texture_floor_l, src, dest)?;
            }

            // Affichage (ou non) du mur d'EN FACE en fonction de player_pos
            let (front_left_x, front_left_y) = match facing {
                0 => (x as i32 - 1, y as i32 - 1),
                1 => (x as i32 + 1, y as i32 - 1),
                2 => (x as i32 + 1, y as i32 + 1),
                3 => (x as i32 - 1, y as i32 + 1),
                _ => (x as i32, y as i32),
            };

            if front_left_x >= 0 && front_left_y >= 0
                && front_left_x < map_width as i32
                && front_left_y < map_height as i32
            {
                let (r, g, b) = get_pixel(
                    loaded_map_array,
                    front_left_x as usize,
                    front_left_y as usize,
                );

                if is_empty_tile(r, g, b) {
                    canvas.copy(&texture_front_l, src, dest)?;
                }
            }
        }



        // CASE player_pos(x+1, y)

        // pixel à droite du joueur (player_pos(x+1, y))
        let (right_x, right_y) = match facing {
            0 => (x as i32 + 1, y as i32),
            1 => (x as i32, y as i32 + 1),
            2 => (x as i32 - 1, y as i32),
            3 => (x as i32, y as i32 - 1),
            _ => (x as i32, y as i32),
        };

        if right_x >= 0 && right_y >= 0 && right_x < map_width as i32 && right_y < map_height as i32 {

            let (rr, rg, rb) = get_pixel(
                loaded_map_array,
                right_x as usize,
                right_y as usize,
            );

            let material_right = get_material(rr, rg, rb);

            // Déterminer le matériau
            let floor_path_r = format!("../textures/{}/{}/floor_x+1y0.png", material_right, facing_str);
            let ceiling_path_r = format!("../textures/{}/{}/ceiling_x+1y0.png", material_right, facing_str);
            let front_path_r = format!("../textures/{}/{}/Fwall_x+1y0.png", material_right, facing_str);

            let texture_floor_r = texture_manager.load(&floor_path_r)?;
            let texture_ceiling_r = texture_manager.load(&ceiling_path_r)?;
            let texture_front_r = texture_manager.load(&front_path_r)?;

            // Affichage du sol et du plafond
            if is_not_empty_tile(rr, rg, rb) {
                canvas.copy(&texture_ceiling_r, src, dest)?;
                canvas.copy(&texture_floor_r, src, dest)?;
            }

            // Affichage (ou non) du mur d'EN FACE en fonction de player_pos
            let (front_right_x, front_right_y) = match facing {
                0 => (x as i32 + 1, y as i32 - 1),
                1 => (x as i32 + 1, y as i32 + 1),
                2 => (x as i32 - 1, y as i32 + 1),
                3 => (x as i32 - 1, y as i32 - 1),
                _ => (x as i32, y as i32),
            };

            if front_right_x >= 0 && front_right_y >= 0
                && front_right_x < map_width as i32
                && front_right_y < map_height as i32
            {
                let (r, g, b) = get_pixel(
                    loaded_map_array,
                    front_right_x as usize,
                    front_right_y as usize,
                );

                if is_empty_tile(r, g, b) {
                    canvas.copy(&texture_front_r, src, dest)?;
                }
            }
        }



        // CASE player_pos(x, y-1)

        // pixel en face du joueur (player_pos(x, y-1))
        let (forward_x, forward_y) = match facing {
            0 => (x as i32, y as i32 - 1),
            1 => (x as i32 + 1, y as i32),
            2 => (x as i32, y as i32 + 1),
            3 => (x as i32 - 1, y as i32),
            _ => (x as i32, y as i32),
        };

        if forward_x >= 0
            && forward_y >= 0
            && forward_x < map_width as i32
            && forward_y < map_height as i32
        {
            let (fr, fg, fb) = get_pixel(
               loaded_map_array,
                forward_x as usize,
                forward_y as usize,
            );

            let material_front = get_material(fr, fg, fb);

            // Déterminer le matériau
            let floor_path_f = format!("../textures/{}/{}/floor_x0y-1.png", material_front, facing_str);
            let ceiling_path_f = format!("../textures/{}/{}/ceiling_x0y-1.png", material_front, facing_str);
            let left_path_f = format!("../textures/{}/{}/Lwall_x0y-1.png", material_front, facing_str);
            let right_path_f = format!("../textures/{}/{}/Rwall_x0y-1.png", material_front, facing_str);
            let front_path_f = format!("../textures/{}/{}/Fwall_x0y-1.png", material_front, facing_str);

            let texture_floor_f = texture_manager.load(&floor_path_f)?;
            let texture_ceiling_f = texture_manager.load(&ceiling_path_f)?;
            let texture_left_f = texture_manager.load(&left_path_f)?;
            let texture_right_f = texture_manager.load(&right_path_f)?;
            let texture_front_f = texture_manager.load(&front_path_f)?;

            // Affichage du sol et du plafond
            if is_not_empty_tile(fr, fg, fb) {
                canvas.copy(&texture_ceiling_f, src, dest)?;
                canvas.copy(&texture_floor_f, src, dest)?;
            }
            
            // Affichage (ou non) du mur d'EN FACE en fonction de player_pos
            let (front2_x, front2_y) = match facing {
                0 => (x as i32, y as i32 - 2),
                1 => (x as i32 + 2, y as i32),
                2 => (x as i32, y as i32 + 2),
                3 => (x as i32 - 2, y as i32),
                _ => (x as i32, y as i32),
            };

            if front2_x >= 0 && front2_y >= 0 && front2_x < map_width as i32 && front2_y < map_height as i32 {
                let (r, g, b) = get_pixel(loaded_map_array, front2_x as usize, front2_y as usize);
                if is_empty_tile(r, g, b) {
                    canvas.copy(&texture_front_f, src, dest)?;
                }
            }

            // Affichage (ou non) du mur de GAUCHE en fonction de player_pos
            let (left2_x, left2_y) = match facing {
                0 => (x as i32 - 1, y as i32 - 1),
                1 => (x as i32 + 1, y as i32 - 1),
                2 => (x as i32 + 1, y as i32 + 1),
                3 => (x as i32 - 1, y as i32 + 1),
                _ => (x as i32, y as i32),
            };

            if left2_x >= 0 && left2_y >= 0 && left2_x < map_width as i32 && left2_y < map_height as i32 {
                let (r, g, b) = get_pixel(loaded_map_array, left2_x as usize, left2_y as usize);
                if is_empty_tile(r, g, b) {
                    canvas.copy(&texture_left_f, src, dest)?;
                }
            }

            // Affichage (ou non) du mur de DROITE en fonction de player_pos
            let (right2_x, right2_y) = match facing {
                0 => (x as i32 + 1, y as i32 - 1),
                1 => (x as i32 + 1, y as i32 + 1),
                2 => (x as i32 - 1, y as i32 + 1),
                3 => (x as i32 - 1, y as i32 - 1),
                _ => (x as i32, y as i32),
            };

            if right2_x >= 0 && right2_y >= 0 && right2_x < map_width as i32 && right2_y < map_height as i32 {
                let (r, g, b) = get_pixel(loaded_map_array, right2_x as usize, right2_y as usize);
                if is_empty_tile(r, g, b) {
                    canvas.copy(&texture_right_f, src, dest)?;
                }
            }
        }



        // CASE player_pos(x, y)

        // Pixel sous le joueur
        let (pr, pg, pb) = get_pixel(loaded_map_array, x, y);
        let material = get_material(pr, pg, pb);

        // Déterminer le matériau
        let floor_path = format!("../textures/{}/{}/floor_x0y0.png", material, facing_str);
        let ceiling_path = format!("../textures/{}/{}/ceiling_x0y0.png", material, facing_str);
        let texture_left_path = format!("../textures/{}/{}/Lwall_x0y0.png", material, facing_str);
        let texture_right_path = format!("../textures/{}/{}/Rwall_x0y0.png", material, facing_str);
        let texture_front_path = format!("../textures/{}/{}/Fwall_x0y0.png", material, facing_str);

        let texture_floor = texture_manager.load(&floor_path)?;
        let texture_ceiling = texture_manager.load(&ceiling_path)?;
        let texture_left = texture_manager.load(&texture_left_path)?;
        let texture_right = texture_manager.load(&texture_right_path)?;
        let texture_front = texture_manager.load(&texture_front_path)?;

        // Affichage du sol et du plafond
        canvas.copy(&texture_ceiling, src, dest)?;
        canvas.copy(&texture_floor, src, dest)?;

        // Affichage (ou non) du mur d'EN FACE en fonction de player_pos
        let (front_x, front_y) = match facing {
            0 => (x as i32, y as i32 - 1),
           1 => (x as i32 + 1, y as i32),
            2 => (x as i32, y as i32 + 1),
            3 => (x as i32 - 1, y as i32),
            _ => (x as i32, y as i32),
        };

        if front_x >= 0 && front_y >= 0 && front_x < map_width as i32 && front_y < map_height as i32 {
            let (r, g, b) = get_pixel(
                loaded_map_array,
                front_x as usize,
                front_y as usize,
            );

           if is_empty_tile(r, g, b) {
                canvas.copy(&texture_front, src, dest)?;
            }
        }

        // Affichage (ou non) du mur de GAUCHE en fonction de player_pos
        let (left_x, left_y) = match facing {
            0 => (x as i32 - 1, y as i32),
            1 => (x as i32, y as i32 - 1),
            2 => (x as i32 + 1, y as i32),
            3 => (x as i32, y as i32 + 1),
            _ => (x as i32, y as i32),
        };

        if left_x >= 0 && left_y >= 0 && left_x < map_width as i32 && left_y < map_height as i32 {
            let (r, g, b) = get_pixel(
               loaded_map_array,
                left_x as usize,
                left_y as usize,
            );

            if is_empty_tile(r, g, b) {
                canvas.copy(&texture_left, src, dest)?;
            }
        }

        // Affichage (ou non) du mur de DROITE en fonction de player_pos
        let (right_x, right_y) = match facing {
            0 => (x as i32 + 1, y as i32),
            1 => (x as i32, y as i32 + 1),
            2 => (x as i32 - 1, y as i32),
            3 => (x as i32, y as i32 - 1),
            _ => (x as i32, y as i32),
       };

        if right_x >= 0 && right_y >= 0 && right_x < map_width as i32 && right_y < map_height as i32 {
            let (r, g, b) = get_pixel(
                loaded_map_array,
                right_x as usize,
                right_y as usize,
            );

           if is_empty_tile(r, g, b) {
                canvas.copy(&texture_right, src, dest)?;
            }
        }
    }



    // Draw texte debug
    //let hello_text: String = "Hello World !".to_string(); // Texte debug
    //let surface = _font
    //    .render(&hello_text)
    //    .blended(Color::RGB(0, 0, 0))
    //    .map_err(|e| e.to_string())?;

    //let texture = texture_creator
    //    .create_texture_from_surface(&surface)
    //    .map_err(|e| e.to_string())?;

    //let target = Rect::new(10 as i32, 0 as i32, 200 as u32, 100 as u32);
    //canvas.copy(&texture, None, Some(target))?;

    canvas.present();
    Ok(())
}



fn main() -> Result<(), String> {

    // INIT SDL2

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Création fenêtre
    let window = video_subsystem.window("Minimal Engine", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .expect("failed to build window");
    let mut canvas = window.into_canvas().build().expect("Failed to initialize canvas");

    // Préparations générales rendu
    let texture_creator = canvas.texture_creator();
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);

    // Préparations fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"../fonts/Courier New.ttf");
    let mut _font = ttf_context.load_font(font_path, 128)?;



    // CHARGEMENT DE L'IMAGE MAP

    // Ouverture de l'image
    let loaded_map = ImageReader::open("../maps/map_debug_3.png")
        .unwrap()
        .decode()
        .unwrap();

    // Convertir en RGB
    let loaded_map = loaded_map.to_rgb8();

    let (map_width, map_height) = loaded_map.dimensions();

    // Convertir en tableau ndarray (loaded_map_array[y, x, r/v/b])
    let loaded_map_array = Array3::from_shape_vec(
        (map_width as usize, map_height as usize, 3),
        loaded_map.into_raw(),
    ).unwrap();

    println!("DEBUG_MAP_LOADING =");
    println!("Dimensions: {:?}", loaded_map_array.dim());
    println!("[0, 0]R: {:?}", loaded_map_array[[0, 0, 0]]);
    println!("[0, 0]G: {:?}", loaded_map_array[[0, 0, 1]]);
    println!("[0, 0]B: {:?}", loaded_map_array[[0, 0, 2]]);
    println!("[11, 4]R: {:?}", loaded_map_array[[11, 4, 0]]);
    println!("[11, 4]G: {:?}", loaded_map_array[[11, 4, 1]]);
    println!("[11, 4]B: {:?}", loaded_map_array[[11, 4, 2]]);



    // SPAWN DU JOUEUR

    let mut player_pos = None;

    for y in 0..map_height as usize {
        for x in 0..map_width as usize {
            let (r, g, b) = get_pixel(&loaded_map_array, x, y);
            let facing = 0;

            if is_spawn_tile(r, g, b) {
                player_pos = Some((x, y, facing));
                break;
            }
        }
    }

    println!("
    ");
    println!("DEBUG_PLAYER_SPAWN =");

    match player_pos {
        Some((x, y, facing)) => println!("Player spawn at: ({}, {}, {})", x, y, facing),
        None => println!("No spawn (r: 255, v: 255, b: 255) found on the loaded map !"),
    }



    // BOUCLE EVENT PUMP

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                },



                // INPUTS JOUEUR

                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    if let Some((x, y, player_pos_facing)) = player_pos {

                        // Nouvelle position
                        let mut new_player_pos_x = i32::try_from(x).unwrap();
                        let mut new_player_pos_y = i32::try_from(y).unwrap();
                        let mut new_player_pos_facing = player_pos_facing;

                        match key {
                            Keycode::Z => {
                                if new_player_pos_facing == (0) {new_player_pos_y -= 1;} // avancer (facing 0/-y)
                                else if new_player_pos_facing == (1) {new_player_pos_x += 1;} // avancer (facing 1/+x)
                                else if new_player_pos_facing == (2) {new_player_pos_y += 1;} // avancer (facing 2/+y)
                                else if new_player_pos_facing == (3) {new_player_pos_x -= 1;} // avancer (facing 3/-x)
                            }
                            Keycode::A => {
                                new_player_pos_facing -= 1; // tourner vers la gauche
                            }
                            Keycode::S => {
                                if new_player_pos_facing == (0) {new_player_pos_y += 1;} // reculer (facing 0/-y)
                                else if new_player_pos_facing == (1) {new_player_pos_x -= 1;} // reculer (facing 1/+x)
                                else if new_player_pos_facing == (2) {new_player_pos_y -= 1;} // reculer (facing 2/+y)
                                else if new_player_pos_facing == (3) {new_player_pos_x += 1;} // reculer (facing 3/-x)
                            }
                            Keycode::E => {
                                new_player_pos_facing += 1; // tourner vers la droite
                            }
                            _ => {}
                        }
                        
                        // délimitation de new_facing[0, 3]
                        if new_player_pos_facing < 0 {new_player_pos_facing = 3}
                        if new_player_pos_facing > 3 {new_player_pos_facing = 0}
                        
                        // Vérification limites de la map
                        if new_player_pos_x >= 0 && new_player_pos_y >= 0 && new_player_pos_x < i32::try_from(map_width).unwrap() && new_player_pos_y < i32::try_from(map_height).unwrap() {

                            // Vérification mur
                                    let (r, g, b) = get_pixel(&loaded_map_array, new_player_pos_x as usize, new_player_pos_y as usize);
                                    if is_not_empty_tile(r, g, b) {
                                        // Autorisation/actualisation de la nouvelle position
                                        player_pos = Some((usize::try_from(new_player_pos_x).unwrap(), usize::try_from(new_player_pos_y).unwrap(), new_player_pos_facing));
                                        println!("DEBUG_PLAYER_POS = New player position at: {:?}", player_pos);
                                    }
                        }
                    }
                }
                _ => {}
            }
        }



        // APPEL DE LA FONCTION DE RENDU

        render(&mut canvas, &mut texture_manager, &texture_creator, &_font, &loaded_map_array, player_pos, map_width, map_height);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
    Ok(())
}

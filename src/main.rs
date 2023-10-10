#![allow(unreachable_code, unused_labels)]

use std::thread::sleep;
use rand::Rng;
use std::thread;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod pieces;

const TAB_WIDTH: usize=12;
const TAB_HEIGH: usize=23;

const WINDOW_WIDTH: usize=690;
const WINDOW_HEIGH: usize=1200;

const PIECES: [[[[u8; 4]; 4]; 4]; 7] = pieces::PIECES;
fn main() {
    println!("Hello, Tetris!");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("tetris", WINDOW_WIDTH as u32, WINDOW_HEIGH as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    boucle_de_jeu(&mut canvas, &mut event_pump);
}

fn boucle_de_jeu(canvas: &mut WindowCanvas, event_pump: &mut sdl2::EventPump){
    let mut board:[[u8;TAB_WIDTH]; TAB_HEIGH] = [[0;TAB_WIDTH];TAB_HEIGH];
    // Remplir les bords gauche et droite avec 2
    for i in 2..TAB_HEIGH {
        board[i][0] = 2;      // Bord gauche
        board[i][TAB_WIDTH-1] = 2;     // Bord droit
    }

// Remplir le bord en bas avec 2
    for j in 0..TAB_WIDTH {
        board[TAB_HEIGH-1][j] = 2;     // Bord en bas
    }
    let mut have_piece:bool = false;
    let mut piece = Tetromino::new(1, 3, 0, 0);
    rotate(&mut piece, &mut board);
    let mut paused:bool=false;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    rotate(&mut piece, &mut board)
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    move_side(&mut board, 1, &mut piece)
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    move_side(&mut board, 0, &mut piece)
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    while down_piece(&mut board, &mut piece) {

                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;  // <- L'utiliser ici devrait fonctionner
                },
                _ => {}
            }
        }
        if have_piece==false {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..=6);
            spawn_piece(random_number, &mut board);
            piece = Tetromino::new(random_number, 3, 0, 0);
            have_piece=true;
            while erase_lines(&mut board) {

            }


        }else{
            if !down_piece(&mut board,&mut piece) {
                if piece.get_y()<3 {
                    break;
                }
                have_piece=false;
            }

            let sleep_duration = Duration::from_millis(200);
            thread::sleep(sleep_duration);
        }
        afficher_tab_gl(board, canvas, event_pump,paused);
    }

}

fn erase_lines(board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH]) -> bool {
    let mut lines_to_remove = Vec::new();

    // Parcours du tableau pour identifier les lignes à supprimer
    for i in 1..(TAB_HEIGH - 1) {
        let mut line_is_complete = true;
        for j in 1..(TAB_WIDTH - 1) {
            if board[i][j] == 0 {
                line_is_complete = false;
                break;
            }
        }
        if line_is_complete {
            lines_to_remove.push(i);
        }
    }

    // Suppression de toutes les lignes identifiées
    for &line_to_remove in lines_to_remove.iter().rev() {
        for i in (1..line_to_remove).rev() {
            for j in 1..(TAB_WIDTH - 1) {
                board[i + 1][j] = board[i][j];
            }
        }
        // Remplacer la ligne supprimée par une nouvelle ligne de zéros
        for j in 1..(TAB_WIDTH - 1) {
            board[1][j] = 0;
        }
    }

    !lines_to_remove.is_empty() // Retourne true s'il y a eu des lignes supprimées
}




fn will_erase(board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH], piece: Tetromino) -> bool{
    for j in 0..4{
        if piece.get_y()>j {
            for i in 0..4 {
                println!("piece {}", PIECES[piece.get_piece_index()][piece.get_angle()][3-j][i]);
                if  PIECES[piece.get_piece_index()][piece.get_angle()][3-j][i] != 0 && (board[piece.get_y()-j][(piece.get_x()+i as isize) as usize]==3 || board[piece.get_y()-j][(piece.get_x()+i as isize) as usize]==2){
                    return true;
                }
            }
        }
    }
    return false;
}

fn draw_piece(piece: &mut Tetromino, board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH]){
    for j in 0..4{
        if piece.get_y()>=j {
            for i in 0..4 {
                if  PIECES[piece.get_piece_index()][piece.get_angle()][3-j][i] != 0{
                    board[piece.get_y()-j][(piece.get_x()+i as isize) as usize] = PIECES[piece.get_piece_index()][piece.get_angle()][3-j][i];
                }
            }
        }
    }
}

fn rotate(piece: &mut Tetromino, board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH]){
    if piece.get_angle() == 3{
        if will_erase(board, Tetromino::new(piece.get_piece_index(), piece.get_x(), piece.get_y(), 0)){
            return;
        }else{
            clean_old_position(board, piece);
            piece.set_angle(0);
            draw_piece(piece, board);
        }
    }else{
        if will_erase(board, Tetromino::new(piece.get_piece_index(), piece.get_x(), piece.get_y(), piece.get_angle()+1)){
            return;
        }else{
            clean_old_position(board, piece);
            piece.set_angle(piece.get_angle()+1);
            draw_piece(piece, board);
        }
    }
}

fn clean_old_position(board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH], piece: &mut Tetromino){
    for j in 0..4{
        if piece.get_y()>=j {
            for i in 0..4 {
                if  PIECES[piece.get_piece_index()][piece.get_angle()][3-j][i] != 0{
                    board[piece.get_y()-j][(piece.get_x()+i as isize) as usize] = 0;
                }
            }
        }
    }
}


fn move_side(board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH], side: u8, piece: &mut Tetromino){
    println!("{}", piece.get_angle());
    if side != 0 && side !=1 { return; }
    if side==0 { //gauche
        if will_erase(board, Tetromino::new(piece.get_piece_index(), piece.get_x()-1, piece.get_y(), piece.get_angle())){
            return;
        }
        clean_old_position(board, piece);
        piece.set_x(piece.get_x()-1);
    }else if side==1 { //droite
        if will_erase(board, Tetromino::new(piece.get_piece_index(), piece.get_x()+1, piece.get_y(), piece.get_angle())){
            return;
        }
        clean_old_position(board, piece);
        piece.set_x(piece.get_x()+1);
    }
    clean_old_position(board, piece);
    draw_piece(piece, board);

}

fn down_piece(board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH], piece: &mut Tetromino) -> bool{
    println!("{}", will_erase(board, Tetromino::new(piece.get_piece_index(), piece.get_x(), piece.get_y()+1, piece.get_angle())));
    if will_erase(board, Tetromino::new(piece.get_piece_index(), piece.get_x(), piece.get_y()+1, piece.get_angle())) {
        for j in 0..4{
            if piece.get_y()>=j {
                for i in 0..4 {
                    if PIECES[piece.get_piece_index()][piece.get_angle()][3-j][i] == 1 {
                        board[piece.get_y()-j][(piece.get_x()+i as isize) as usize] = 3;
                    }
                }
            }
        }
        return false;
    }
    clean_old_position(board, piece);
    piece.set_y(piece.get_y()+1);
    for j in 0..4{
        if piece.get_y()>=j {
            for i in 0..4 {
                if PIECES[piece.get_piece_index()][piece.get_angle()][3-j][i] == 1 {
                    board[piece.get_y()-j][(piece.get_x()+i as isize) as usize] = 1;
                }
            }
        }
    }
    return true;
}

fn afficher_tab_gl(board: [[u8; TAB_WIDTH]; TAB_HEIGH], canvas: &mut WindowCanvas, event_pump: &mut EventPump, paused: bool){
    if !paused {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for (row_idx, row) in board.iter().enumerate() {
            for (cell_idx, &cell) in row.iter().enumerate() {
                if cell==0 {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }else if cell == 1{
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                }else if cell == 2 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }else if cell == 3 {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                }
                canvas.fill_rect(Rect::new((cell_idx * WINDOW_WIDTH / row.len()) as i32, (row_idx * WINDOW_HEIGH / board.len()) as i32, (WINDOW_HEIGH / board.len()) as u32, (WINDOW_WIDTH / row.len()) as u32)).unwrap();

            }
        }

        canvas.present();
    }
}

fn print_board(board: [[u8; TAB_WIDTH]; TAB_HEIGH]){
    println!("---------------------------------------");
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!()
    }
}
fn spawn_piece(piece: usize, board: &mut [[u8; TAB_WIDTH]; TAB_HEIGH]) {
    for i in 0..4 {
        board[0][i + 3] = PIECES[piece][0][0][i];
    }
}

struct Tetromino {
    i: usize, // Quelle pièce
    x: isize,
    y: usize,
    a: usize, // Quel angle, entre 0 et 3
}

impl Tetromino {
    // Constructeur pour créer une nouvelle instance de Tetromino
    pub fn new(piece_index: usize, x: isize, y: usize, angle: usize) -> Tetromino {
        Tetromino {
            i: piece_index,
            x,
            y,
            a: angle,
        }
    }

    // Getter pour obtenir l'indice de la pièce
    pub fn get_piece_index(&self) -> usize {
        self.i
    }

    // Setter pour modifier l'indice de la pièce
    pub fn set_piece_index(&mut self, piece_index: usize) {
        self.i = piece_index;
    }

    // Getter pour obtenir la position x
    pub fn get_x(&self) -> isize {
        self.x
    }

    // Setter pour modifier la position x
    pub fn set_x(&mut self, x: isize) {
        self.x = x;
    }

    // Getter pour obtenir la position y
    pub fn get_y(&self) -> usize {
        self.y
    }

    // Setter pour modifier la position y
    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    // Getter pour obtenir l'angle
    pub fn get_angle(&self) -> usize {
        self.a
    }

    // Setter pour modifier l'angle
    pub fn set_angle(&mut self, angle: usize) {
        self.a = angle;
    }
}

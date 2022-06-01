use zoon::{*, named_color::*};
use chess::{Board, BoardBuilder, Piece, Square, Color, Game, ALL_PIECES, ALL_SQUARES, ALL_RANKS, ALL_FILES, Rank, File};


// ------ ------
//     View
// ------ ------

fn root() -> impl Element {

    let game = Game::new_with_board(Board::default());
    let board = game.current_position();

    Stack::new()
        .s(Width::fill())
        .s(Height::fill())
        .layer(chessboard(board))

}

fn square_content(piece: Piece, color: Color) -> impl Element {
    El::new()
        .child(
            Paragraph::new()
                .content(
                    Image::new()
                        .s(Height::fill())
                        .url(square_piece(piece, color))
                        .description("piece image")
                )
            .s(Align::center())
            )
}

fn square_piece(piece: Piece, color: Color) -> &'static str {

    match (piece, color) {
        (Piece::Pawn, Color::Black) => "https://upload.wikimedia.org/wikipedia/commons/c/cd/Chess_pdt60.png",
        (Piece::Pawn, Color::White) => "https://upload.wikimedia.org/wikipedia/commons/0/04/Chess_plt60.png",
        (Piece::Knight, Color::Black) => "https://upload.wikimedia.org/wikipedia/commons/f/f1/Chess_ndt60.png",
        (Piece::Knight, Color::White) => "https://upload.wikimedia.org/wikipedia/commons/2/28/Chess_nlt60.png",
        (Piece::Bishop, Color::Black) => "https://upload.wikimedia.org/wikipedia/commons/8/81/Chess_bdt60.png",
        (Piece::Bishop, Color::White) => "https://upload.wikimedia.org/wikipedia/commons/9/9b/Chess_blt60.png",
        (Piece::Rook, Color::Black) => "https://upload.wikimedia.org/wikipedia/commons/a/a0/Chess_rdt60.png",
        (Piece::Rook, Color::White) => "https://upload.wikimedia.org/wikipedia/commons/5/5c/Chess_rlt60.png",
        (Piece::Queen, Color::Black) => "https://upload.wikimedia.org/wikipedia/commons/a/af/Chess_qdt60.png",
        (Piece::Queen, Color::White) => "https://upload.wikimedia.org/wikipedia/commons/4/49/Chess_qlt60.png",
        (Piece::King, Color::Black) => "https://upload.wikimedia.org/wikipedia/commons/e/e3/Chess_kdt60.png",
        (Piece::King, Color::White) => "https://upload.wikimedia.org/wikipedia/commons/3/3b/Chess_klt60.png",
        //(None, _) => "https://upload.wikimedia.org/wikipedia/commons/1/1d/No_image.svg"
    }
}

fn square_background(rank: Rank, file: File) -> HSLuv {

    let mut sum = 0;

    match rank {
        Rank::First =>  sum += 1,
        Rank::Second => sum += 2,
        Rank::Third =>  sum += 3,
        Rank::Fourth => sum += 4,
        Rank::Fifth =>  sum += 5,
        Rank::Sixth =>  sum += 6,
        Rank::Seventh => sum += 7,
        Rank::Eighth => sum += 8,
    }

    match file {
        File::A => sum += 1,
        File::B => sum += 2,
        File::C => sum += 3,
        File::D => sum += 4,
        File::E => sum += 5,
        File::F => sum += 6,
        File::G => sum += 7,
        File::H => sum += 8
    }

    match sum %  2 {
        0 => GREEN_6,
        _ => GREEN_1
    }
}

fn square(board: Board, file: File, rank: Rank) -> impl Element {
    let square = Square::make_square(rank, file);
    let piece = board.piece_on(square);

    let sq = Stack::new()
        .layer(square_el(rank, file));
    
    match piece {
        Some(p) => sq.layer(square_content(p, board.color_on(square).unwrap())),
        None => sq,
    }
}

fn square_el(rank: Rank, file: File) -> impl Element{
    El::new()
        .s(Width::new(64))
        .s(Height::new(64))
        .s(Background::new().color(square_background(rank, file))
    )
}

fn chessboard(board: Board) -> impl Element {

    let mut rows = Vec::new();

    ALL_RANKS.iter().rev().for_each(|rank| {
        let mut row = Vec::new();
        
        ALL_FILES.iter().for_each(|file| {
            row.push(square(board, *file, *rank))
        });

        rows.push(Row::new().items(row));
    });

    Column::new().items(rows).s(Align::center()).s(Width::new(512))

}

fn chessboard1(flip: bool) -> impl Element {
    let (range, label_edge) = match flip {
        false => ([0,1,2,3,4,5,6,7], 0),
        true => ([7,6,5,4,3,2,1,0], 7),
    };
    let file_letters = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let mut rows = Vec::new();

    for rank in range.into_iter().rev() {
        let mut row = Vec::new();
        for file in range {
            let mut square = Stack::new()
                .layer(El::new()
                    .s(Width::new(64))
                    .s(Height::new(64))
                    .s(Background::new().color(
                        match (rank + file) % 2 {
                            0 => GREEN_6,
                            _ => GREEN_1,
                    })
    
                ));
                // rank labels
                if file == label_edge {
                    square = square.layer(Label::new().label(rank + 1)
                        .s(Align::new().top().left())
                        .s(Font::new().color(
                            match (rank + file) % 2 {
                                0 => GREEN_1,
                                _ => GREEN_6,
                        }
                    )));
                }
                // file labels
                if rank == label_edge {
                    square = square.layer(Label::new().label(file_letters[file])
                        .s(Align::new().bottom().right())
                        .s(Font::new().color(
                            match (rank + file) % 2 {
                                0 => GREEN_1,
                                _ => GREEN_6,
                            }
                        )));
                }
            row.push(square);
        }
        rows.push(Row::new().items(row).s(Align::center()));
    }
    
    Column::new().items(rows).s(Align::center()).s(Width::new(512))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("main", root);
}

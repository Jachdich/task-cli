use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Event;
use termion::event::Key;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use json;
use std::io::{Read, Write};

struct Card {
    title: String,
    body: String,
}

struct CardList {
    cards: Vec<Card>,
    title: String,
}

struct Board {
    title: String,
    lists: Vec<CardList>,
}

impl Card {
    fn from_json(value: &json::JsonValue) -> Self {
        Card {
            title: value["title"].as_str().unwrap().to_owned(),
            body:  value["body"].as_str().unwrap().to_owned(),
        }
    }

    fn to_json(&self) -> json::JsonValue {
        json::object!{
            title: self.title.to_owned(),
            body: self.body.to_owned(),
        }
    }
}

impl CardList {
    fn from_json(value: &json::JsonValue) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for member in value["cards"].members() {
            cards.push(Card::from_json(member));
        }
        CardList {
            title: value["title"].as_str().unwrap().to_owned(),
            cards,
        }
    }

    fn to_json(&self) -> json::JsonValue {
        let mut cards = json::array![];
        for card in &self.cards {
            cards.push(card.to_json());
        }
        json::object!{
            title: self.title.to_owned(),
            cards: cards
        }
    }
}

impl Board {
    fn from_json(value: &json::JsonValue) -> Self {
        let mut lists: Vec<CardList> = Vec::new();
        for member in value["lists"].members() {
            lists.push(CardList::from_json(member));
        }
        Board {
            title: value["title"].as_str().unwrap().to_owned(),
            lists,
        }
    }

    fn to_json(&self) -> json::JsonValue {
        let mut lists = json::array![];
        for cardlist in &self.lists {
            lists.push(cardlist.to_json());
        }
        json::object!{
            title: self.title.to_owned(),
            lists: lists
        }
    }
}

fn load(fname: &str) -> std::io::Result<Board> {
    let mut file = std::fs::File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(Board::from_json(&json::parse(&contents).unwrap()))
}

fn save(fname: &str, board: &Board) -> std::io::Result<()> {
    let mut file = std::fs::File::create(fname)?;
    file.write_all(&board.to_json().dump().into_bytes())?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut curr_board = load("test.json").unwrap();
    let mut curr_board_ref = &curr_board;
    let stdin = io::stdin();
    for event in stdin.events() {

        match event.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                break;
            }
            _ => ()
        }

        terminal.clear()?;
        
        terminal.draw(|f| {
            let size = f.size();
            let block = board.draw();
            f.render_widget(block, size);
        })?;
    }
    Ok(())
}
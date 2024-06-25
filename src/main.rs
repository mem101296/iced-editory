//Video:https://www.youtube.com/watch?v=gcBJ7cPSALo&t=879s
//38:12

use iced::executor;
//declairing widget
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor};
//importing iced libraries
use iced::{Application, Command, Element, Length, Settings, Theme};

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    path: Option<PathBuf>, //place holder path for new files or pre-opening a file
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Open,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
}

//sandbox is a simple application
impl Application for Editor {
    //impl Sandbox for Editor {
    type Message = Message;
    type Theme = Theme; //app theme
    type Executor = executor::Default; //engine to run sync tasks in background
    type Flags = (); //data that app initializes

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                path: None,
                //what shows when the editor opens
                content: text_editor::Content::new(),
                error: None,
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("A cool editor!")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            //edit whatever is loaded
            Message::Edit(action) => {
                self.content.edit(action);
                Command::none()
            }

            Message::Open => Command::perform(pick_file(), Message::FileOpened),

            //open file
            Message::FileOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.content = text_editor::Content::with(&content);
                Command::none()
            }

            //output error when file opened
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
        }
    }

    //the actual window
    fn view(&self) -> Element<'_, Message> {
        let controls = row![button("Open").on_press(Message::Open)];
        let input = text_editor(&self.content).on_edit(Message::Edit);

        let file_path = match self.path.as_ref().map(Path::to_str) {
            Some(path) => text(path).size(14),
            None => text(""),
        };

        let position = {
            //cursor position
            let (line, column) = self.content.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };

        //status bar at bottom
        let status_bar = row![horizontal_space(Length::Fill), position];

        //padding of the window
        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    //theme
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

//shows a dialoge to pick a file
async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;

    Ok((path, contents))
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

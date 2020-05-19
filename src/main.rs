#![recursion_limit = "2048"]

use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

use commands::{man2html, manpages, manpath};

use ext::*;

mod commands;
mod ext;

#[derive(Clone, Debug)]
struct Model {
    paths: Vec<String>,
    pages: Vec<String>,
    filter: Option<String>,
    manpage: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            paths: vec![],
            pages: vec![],
            filter: None,
            manpage: include_str!("index.html").to_string(),
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    Search(String),
    SearchResult(Vec<String>),
    NoOp,
    LoadManpage(String),
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::NoOp => UpdateAction::None,
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::LoadManpage(path) => {
                self.manpage = man2html(&path).expect("LoadManpage failed!");
                UpdateAction::Render
            }
            Message::Search(search_string) => UpdateAction::defer(async {
                let paths = manpath();
                let pages = manpages(&paths)
                    .iter()
                    .filter(move |item| item.contains(&search_string))
                    .map(|item| item.to_owned())
                    .collect::<Vec<_>>();

                Message::SearchResult(pages)
            }),
            Message::SearchResult(res) => {
                self.pages = res;
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("com.github.lihram.vgtk-manpages"), ApplicationFlags::empty())>
                <ApplicationWindow default_width=800 default_height=480 border_width=0 on destroy=|_| Message::Exit>
                    <Paned>
                        <Box orientation=Orientation::Vertical Paned::shrink=false>
                            <SearchBar search_mode=true>
                                <SearchEntry placeholder_text="Ask, and you shall receive." on activate=|entry| {
                                    let input = entry.get_text().map(|s| s.to_string()).unwrap_or_default();
                                    match input.is_empty() {
                                        true => Message::NoOp,
                                        false => Message::Search(input)
                                    }
                                }/>
                            </SearchBar>
                            <ScrolledWindow Box::fill=true Box::expand=true>
                                <ListBox>
                                    {
                                        self.pages.iter().map(|page| gtk! {
                                            <ListBoxRow>
                                                <LinkButton uri=page.clone() on clicked=|el| Message::LoadManpage(el.get_uri().map(|s| s.to_string()).unwrap_or_default())>
                                                    <Label xalign=0.0 label=get_name(&page) />
                                                </LinkButton>
                                            </ListBoxRow>
                                        })
                                    }
                                </ListBox>
                            </ScrolledWindow>
                        </Box>
                        <Box Paned::shrink=true>
                            <WebView html=self.manpage.clone() Box::expand=true />
                        </Box>
                    </Paned>
                </ApplicationWindow>
            </Application>
        }
    }

    fn mounted(&mut self) {}

    fn unmounted(&mut self) {}
}

fn get_name(path: &str) -> String {
    path.split("/").last().unwrap().to_string()
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}

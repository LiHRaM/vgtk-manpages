#![recursion_limit = "2048"]

use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, run, Component, UpdateAction, VNode};

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
        let paths = manpath();
        let pages = manpages(&paths);
        let manpage = man2html(pages.first().unwrap()).unwrap();

        Self {
            paths,
            pages,
            filter: None,
            manpage,
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    FilterPages(Option<String>),
    LoadManpage(String),
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::FilterPages(filter) => {
                self.filter = filter;
                UpdateAction::Render
            }
            Message::LoadManpage(path) => {
                self.manpage = man2html(&path).expect("LoadManpage failed!");
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
                                        true => Message::FilterPages(None),
                                        false => Message::FilterPages(Some(input))
                                    }
                                }/>
                            </SearchBar>
                            <ScrolledWindow Box::fill=true Box::expand=true>
                                <ListBox>
                                    {
                                        self.filter(self.filter.clone()).map(|page| gtk! {
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
                        {
                            gtk_if!( !self.manpage.is_empty() => {
                                <Box>
                                    <WebView html=self.manpage.clone() Box::expand=true />
                                </Box>
                            })
                        }
                    </Paned>
                </ApplicationWindow>
            </Application>
        }
    }
}

impl<'a> Model {
    fn filter(&self, filter: Option<String>) -> impl Iterator<Item = &String> {
        self.pages.iter().filter(move |item| match filter {
            Some(ref n) => item.contains(n),
            None => true,
        })
    }
}

fn get_name(path: &str) -> String {
    path.split("/").last().unwrap().to_string()
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}

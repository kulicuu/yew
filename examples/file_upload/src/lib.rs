#![recursion_limit = "256"]


use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Model>,
    tasks: Vec<ReaderTask>,
    files: Vec<String>,
    by_chunks: bool,
}

type Chunks = bool;

pub enum Msg {
    Loaded(FileData),
    Chunk(Option<FileChunk>),
    Files(Vec<File>, Chunks),
    ToggleByChunks,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            tasks: vec![],
            files: vec![],
            by_chunks: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(file) => {
                let info = format!("file: {:?}", file);
                self.files.push(info);
            }
            Msg::Chunk(Some(chunk)) => {
                let info = format!("chunk: {:?}", chunk);
                self.files.push(info);
            }
            Msg::Files(files, chunks) => {
                let mut reader_svc = ReaderService::new();
                for file in files.into_iter() {
                    let task = {
                        if chunks {
                            let callback = self.link.callback(Msg::Chunk);
                            ReaderService::read_file_by_chunks(&mut reader_svc, file, callback, 10).unwrap()
                        } else {
                            let callback = self.link.callback(Msg::Loaded);
                            ReaderService::read_file(&mut reader_svc, file, callback).unwrap()
                        }
                    };
                    self.tasks.push(task);
                }
            }
            Msg::ToggleByChunks => {
                self.by_chunks = !self.by_chunks;
            }
            _ => return false,
        }
        true
    }

    fn view(&self) -> Html {
        let flag = self.by_chunks;
        html! {
            <div>
                <div>
                    <p>{"Choose a file to upload to see the uploaded bytes"}</p>
                    <input type="file" multiple=true onchange=self.link.callback(move |value| {
                            let mut result = Vec::new();
                            if let ChangeData::Files(files) = value {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .map(|v| File::from(v.unwrap()));
                                result.extend(files);
                            }
                            Msg::Files(result, flag)
                        })/>
                </div>
                <div>
                    <label>{ "By chunks" }</label>
                    <input type="checkbox" checked=flag onclick=self.link.callback(|_| Msg::ToggleByChunks) />
                </div>
                <ul>
                    { for self.files.iter().map(|f| self.view_file(f)) }
                </ul>
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Model {
    fn view_file(&self, data: &str) -> Html {
        html! {
            <li>{ data }</li>
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
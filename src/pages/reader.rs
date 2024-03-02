use crate::fetcher::{Complete, Fetcher};
use crate::get_app_data;
use api_structure::image::MangaReaderImageRequest;
use api_structure::reader::{
    Action, MangaReaderRequest, MangaReaderResponse, ReaderChapter, ReaderPageRequest,
    ReaderPageResponse,
};
use api_structure::{now_timestamp, RequestImpl};
use eframe::{App, Frame};
use egui::{vec2, Context, Image, Ui, Vec2};
use ethread::ThreadHandler;
use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

type PageData = HashMap<String, Fetcher<Arc<ReaderPageResponse>>>;

pub struct MangaReaderPage {
    storage: Storage,
    settings: Settings,
    progress: Option<Progress>,
    init: bool,
}

struct Progress {
    chapter: String,
    image: u32,
    pixels: f32,
}

struct Storage {
    manga: Fetcher<Arc<MangaReaderResponse>>,
    page_data: PageData,
    loaded_pages: ImageStorage,
}

impl Storage {
    fn get_page_data() {}
}

enum State {
    ChapterLoading,
    ChapterError,
    ReaderPageResponse(Arc<ReaderPageResponse>),
    NoChapter,
}

impl MangaReaderPage {
    pub(crate) fn new(manga_id: String, chapter_id: Option<String>) -> Self {
        let mut manga = Fetcher::new(MangaReaderRequest::request(&get_app_data().url).unwrap());
        manga.set_body(MangaReaderRequest {
            manga_id,
            chapter_id,
        });
        manga.send();
        Self {
            storage: Storage {
                manga,
                page_data: Default::default(),
                loaded_pages: Default::default(),
            },
            settings: Settings {
                version_hierachy: vec![],
                reading_mode: ReadingMode::Single,
                prefetch: 3.0,
                view_area: ViewArea {
                    margin_top: 0.0,
                    margin_right: 0.0,
                    margin_bottom: 0.0,
                    margin_left: 0.0,
                },
            },
            progress: None,
            init: false,
        }
    }
}

impl App for MangaReaderPage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if !self.init {
            self.init = true;
            self.storage.manga.set_ctx(ctx.clone());
        }

        let app = get_app_data();
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(v) = self.storage.manga.result().cloned() {
                match v {
                    Complete::Json(v) => {
                        if let Some(p) = &self.progress {
                            load_images(
                                v.clone(),
                                &self.settings.version_hierachy,
                                &mut self.storage.page_data,
                                &p.chapter,
                                ctx,
                                &self.settings.reading_mode,
                                self.settings.prefetch,
                                p,
                                self.settings.view_area.get_size(ui),
                                &mut self.storage.loaded_pages,
                            );
                            display_images(
                                v.clone(),
                                &self.settings.version_hierachy,
                                &mut self.storage.page_data,
                                p,
                                ui,
                                &self.settings.reading_mode,
                            )
                        } else {
                            if v.no_chapters() {
                                //TODO: no chapters
                            } else {
                                let ver = get_version(
                                    v.get_chapter(&v.open_chapter).unwrap(),
                                    &self.settings.version_hierachy,
                                );
                                self.progress = Some(Progress {
                                    chapter: v.open_chapter.to_string(),
                                    image: 1,
                                    pixels: 0.0,
                                });
                                //TODO: create progress
                            }
                        }
                    }
                    _ => v.display_error(ui),
                }
            }
        });
    }
}

/// gets infos for pages like dimensions
fn get_page_resp(
    v: Arc<MangaReaderResponse>,
    hierachy: &[String],
    page_data: &mut PageData,
    chapter: &str,
    ctx: &Context,
) -> State {
    let ver = get_version(v.get_chapter(chapter).unwrap(), hierachy);
    if let Some(ver) = ver {
        if let Some(v) = page_data.get_mut(ver) {
            return if let Some(v) = v.result() {
                match v {
                    Complete::Json(cv) => State::ReaderPageResponse(cv.clone()),
                    _ => State::ChapterError,
                }
            } else {
                State::ChapterLoading
            };
        }
        let mut fetcher = Fetcher::new(ReaderPageRequest::request(&get_app_data().url).unwrap());
        fetcher.set_ctx(ctx.clone());
        fetcher.set_body(&ReaderPageRequest {
            chapter_version_id: ver.to_string(),
        });
        fetcher.send();
        page_data.insert(ver.to_string(), fetcher);
        return get_page_resp(v, hierachy, page_data, chapter, ctx);
    }
    State::NoChapter
}

/// gets prioritized  version
fn get_version<'a>(rc: &'a ReaderChapter, hierachy: &[String]) -> Option<&'a String> {
    let key = rc.versions.keys().next();
    if key.is_none() {
        return None;
    }
    for id in hierachy {
        if let Some(v) = rc.versions.get(id) {
            return Some(v);
        }
    }
    rc.versions.get(key.unwrap())
}

fn get_version_key(rc: &ReaderChapter, hierachy: &[String]) -> Option<String> {
    let key = rc.versions.keys().next();
    if key.is_none() {
        return None;
    }
    for id in hierachy {
        return Some(id.clone());
    }
    return key.cloned();
}

#[derive(Default)]
struct ImageStorage {
    hashmap: HashMap<String, ImageStore>,
}

impl ImageStorage {
    fn get(&mut self, s: &str) -> Option<&ImageStore> {
        if let Some(v) = self.hashmap.get_mut(s) {
            v.last_checked = now_timestamp().unwrap();
            Some(v)
        } else {
            None
        }
    }

    fn insert(&mut self, key: String, image: ThreadHandler<Option<Image<'static>>>) {
        self.hashmap.insert(
            key,
            ImageStore {
                last_checked: Default::default(),
                req: image,
            },
        );
    }
}

struct ImageStore {
    last_checked: Duration,
    req: ThreadHandler<Option<Image<'static>>>,
}

/// load images in range
fn load_images(
    v: Arc<MangaReaderResponse>,
    hierachy: &[String],
    page_data: &mut PageData,
    chapter: &str,
    ctx: &Context,
    rm: &ReadingMode,
    area: f32,
    progress: &Progress,
    size: Vec2,
    imgs: &mut ImageStorage,
) {
    let reader_page = if let State::ReaderPageResponse(v) =
        get_page_resp(v.clone(), hierachy, page_data, chapter, ctx)
    {
        v
    } else {
        return;
    };
    //let mut before_reader_page = None;
    // let mut after_reader_page = None;

    match rm {
        ReadingMode::Single => single(&v, hierachy, chapter, area, progress, &reader_page, 1, imgs),
        ReadingMode::Double(_) => {
            single(&v, hierachy, chapter, area, progress, &reader_page, 2, imgs)
        }
        ReadingMode::Strip => multi(
            &v,
            hierachy,
            chapter,
            area,
            progress,
            &reader_page,
            size.x,
            size.y,
            true,
            imgs,
        ),
        ReadingMode::Row(_) => multi(
            &v,
            hierachy,
            chapter,
            area,
            progress,
            &reader_page,
            size.y,
            size.x,
            false,
            imgs,
        ),
    }
}

fn multi(
    v: &Arc<MangaReaderResponse>,
    hierachy: &[String],
    chapter: &str,
    area: f32,
    progress: &Progress,
    reader_page: &Arc<ReaderPageResponse>,
    x: f32,
    y: f32,
    height: bool,
    imgs_: &mut ImageStorage,
) {
    let area = y * area;
    let mut processed = 0.;
    let mut count = 0;
    while processed < area + y {
        let (br, img) = load_image(&v, hierachy, chapter, progress, imgs_, &reader_page, count);
        match img {
            Action::Page(v) => {
                let a = if height { v.height(x) } else { v.width(x) };
                processed += a;
            }
            _ => {}
        }
        if br {
            break;
        }
        count += 1;
    }
}

fn single(
    v: &Arc<MangaReaderResponse>,
    hierachy: &[String],
    chapter: &str,
    area: f32,
    progress: &Progress,
    reader_page: &Arc<ReaderPageResponse>,
    i_: u32,
    imgs_: &mut ImageStorage,
) {
    let area = area as u32 * i_;
    for i in 0..=area + i_ {
        if load_image(&v, hierachy, chapter, progress, imgs_, &reader_page, i).0 {
            break;
        }
        if i != 0 && i <= area {
            if load_image(&v, hierachy, chapter, progress, imgs_, &reader_page, i).0 {
                break;
            }
        }
    }
}

fn load_image<'a>(
    v: &Arc<MangaReaderResponse>,
    hierachy: &[String],
    chapter: &str,
    progress: &Progress,
    imgs: &mut ImageStorage,
    rp: &'a Arc<ReaderPageResponse>,
    offset: u32,
) -> (bool, Action<'a>) {
    let p = rp.get_page(progress.image + offset);
    let cont = match &p {
        Action::Prev => {
            //TODO: load
            true
        }
        Action::Page(p) => {
            if imgs.get(&p.page_id).is_none() {
                let ver = get_version_key(v.get_chapter(chapter).unwrap(), hierachy);
                if let Some(ver) = ver {
                    let data = MangaReaderImageRequest {
                        manga_id: v.manga_id.clone(),
                        chapter_id: chapter.to_string(),
                        version_id: ver,
                        page: progress.image + offset,
                        file_ext: p.ext.clone(),
                    };
                    let page_id = p.page_id.clone();

                    imgs.insert(
                        page_id.clone(),
                        ThreadHandler::new_async(async move {
                            let token = format!(
                                "Bearer {}",
                                get_app_data().get_access_token().await.unwrap()
                            );
                            let res = get_app_data()
                                .client
                                .post(get_app_data().url.join("chapter_page").unwrap())
                                .header(AUTHORIZATION, token)
                                .json(&data)
                                .send()
                                .await
                                .ok()?
                                .bytes()
                                .await
                                .ok()?;
                            Some(Image::from_bytes(
                                format!("bytes://manga_image_{}", page_id),
                                res.to_vec(),
                            ))
                        }),
                    )
                }
            }
            false
        }
        Action::Next => true,
    };
    (cont, p)
}

fn max(img: Vec2, area: Vec2) -> Vec2 {
    let scale_x = area.x / img.x;
    let scale_y = area.y / img.y;
    let min_scale = scale_x.min(scale_y);
    vec2(img.x * min_scale, img.y * min_scale)
}

/// displays images
fn display_images(
    v: Arc<MangaReaderResponse>,
    hierachy: &[String],
    page_data: &mut PageData,
    progress: &Progress,
    ui: &mut Ui,
    rm: &ReadingMode,
) {
    let mut chapter = progress.chapter.clone();
    let ch = get_page_resp(v, hierachy, page_data, &chapter, ui.ctx());
    match rm {
        ReadingMode::Single => match ch {
            State::ReaderPageResponse(v_) => match v_.get_page(progress.image) {
                Action::Page(v) => {}
                _ => unreachable!(),
            },
            _ => {}
        },
        ReadingMode::Double(_) => unimplemented!(),
        ReadingMode::Strip => unimplemented!(),
        ReadingMode::Row(_) => unimplemented!(),
    }

    let mut pos = 0.;
}

struct Settings {
    reading_mode: ReadingMode,
    prefetch: f32,
    view_area: ViewArea,
    version_hierachy: Vec<String>,
}

struct ViewArea {
    margin_top: f32,
    margin_right: f32,
    margin_bottom: f32,
    margin_left: f32,
}

impl ViewArea {
    fn get_size(&self, ui: &mut Ui) -> Vec2 {
        ui.available_size()
            - vec2(
                self.margin_right + self.margin_left,
                self.margin_top + self.margin_bottom,
            )
    }
}

enum ReadingMode {
    Single,
    /// bool for left to right
    Double(bool),
    Strip,
    /// bool for left to right
    Row(bool),
}

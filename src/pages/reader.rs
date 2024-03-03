use crate::fetcher::{Complete, Fetcher};
use crate::fonts::Fonts;
use crate::get_app_data;
use api_structure::image::MangaReaderImageRequest;
use api_structure::reader::{
    Action, MangaReaderRequest, MangaReaderResponse, ReaderChapter, ReaderPage, ReaderPageRequest,
    ReaderPageResponse, TranslationArea,
};
use api_structure::{now_timestamp, RequestImpl};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use eframe::{App, Frame};
use egui::text::LayoutJob;
use egui::{
    include_image, vec2, Align, Align2, Color32, Context, Direction, FontFamily, FontId, Image,
    Label, Layout, Rect, TextFormat, Ui, Vec2, WidgetText,
};
use ethread::ThreadHandler;
use image::EncodableLayout;
use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

const TEXT_SHADOWS: [[f32; 2]; 16] = [
    [1.5 * 1., 1.5 * 0.],
    [1.5 * 0.924, 1.5 * 0.383],
    [1.5 * 0.707, 1.5 * 0.707],
    [1.5 * 0.383, 1.5 * 0.924],
    [1.5 * 0., 1.5 * 1.],
    [1.5 * -0.383, 1.5 * 0.924],
    [1.5 * -0.707, 1.5 * 0.707],
    [1.5 * -0.924, 1.5 * 0.3827],
    [1.5 * -1., 1.5 * 0.],
    [1.5 * -0.924, 1.5 * -0.383],
    [1.5 * -0.707, 1.5 * -0.707],
    [1.5 * -0.383, 1.5 * -0.924],
    [1.5 * 0., 1.5 * -1.],
    [1.5 * 0.383, 1.5 * -0.924],
    [1.5 * 0.707, 1.5 * -0.707],
    [1.5 * 0.924, 1.5 * -0.383],
];

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

#[derive(Clone)]
enum State {
    ChapterLoading,
    ChapterError,
    ReaderPageResponse(Arc<ReaderPageResponse>),
    NoChapter,
}

enum StateWithImage {
    ChapterLoading,
    ChapterError,
    ReaderPageResponse((Option<ReaderPage>, Arc<Image<'static>>)),
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
                reading_mode: ReadingMode::Double(false),
                prefetch: 3.0,
                view_area: ViewArea {
                    margin_top: 20.0,
                    margin_right: 0.0,
                    margin_bottom: 20.0,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(v) = self.storage.manga.result().cloned() {
                match v {
                    Complete::Json(v) => {
                        if let Some(p) = &self.progress {
                            let size = self.settings.view_area.get_size(ctx);
                            load_images(
                                v.clone(),
                                &self.settings.version_hierachy,
                                &mut self.storage.page_data,
                                &p.chapter,
                                ctx,
                                &self.settings.reading_mode,
                                self.settings.prefetch,
                                p,
                                size,
                                &mut self.storage.loaded_pages,
                            );
                            display_images(
                                v.clone(),
                                &self.settings.version_hierachy,
                                &mut self.storage.page_data,
                                p,
                                ui,
                                &self.settings.reading_mode,
                                &mut self.storage.loaded_pages,
                                size,
                                vec2(
                                    self.settings.view_area.margin_left,
                                    self.settings.view_area.margin_top,
                                ),
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

struct ImageStorage {
    hashmap: HashMap<String, ImageStore>,
    loading: Arc<(Image<'static>, Vec<ReaderTranslationArea>)>,
    error: Arc<(Image<'static>, Vec<ReaderTranslationArea>)>,
}

impl Default for ImageStorage {
    fn default() -> Self {
        Self {
            hashmap: Default::default(),
            loading: Arc::new((
                get_app_data().spinner.lock().unwrap().clone().unwrap(),
                vec![],
            )),
            error: Arc::new((Image::from(include_image!("../assets/error.gif")), vec![])),
        }
    }
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

    fn insert(
        &mut self,
        key: String,
        image: ThreadHandler<Option<Arc<(Image, Vec<ReaderTranslationArea>)>>>,
    ) {
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
    req: ThreadHandler<Option<Arc<(Image<'static>, Vec<ReaderTranslationArea>)>>>,
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
                    let fetch_trans = p.translation;

                    imgs.insert(
                        page_id.clone(),
                        ThreadHandler::new_async(async move {
                            let token = format!(
                                "Bearer {}",
                                get_app_data().get_access_token().await.unwrap()
                            );
                            let mut translations = vec![];
                            if fetch_trans {
                                let mut t: Vec<TranslationArea> = serde_json::from_slice(
                                    get_app_data()
                                        .client
                                        .post(get_app_data().url.join("page_translation").unwrap())
                                        .header(AUTHORIZATION, &token)
                                        .json(&data)
                                        .send()
                                        .await
                                        .ok()?
                                        .bytes()
                                        .await
                                        .ok()?
                                        .as_bytes(),
                                )
                                .ok()?;
                                for (index, trans) in t.into_iter().enumerate() {
                                    let back = trans
                                        .background
                                        .split_once(";base64,")
                                        .map(|v| v.1.to_string())
                                        .unwrap_or_else(|| trans.background);
                                    translations.push(ReaderTranslationArea {
                                        translated_text: trans.translated_text,
                                        min_x: trans.min_x,
                                        min_y: trans.min_y,
                                        max_x: trans.max_x,
                                        max_y: trans.max_y,
                                        text_color: Color32::BLACK,
                                        outline_color: Color32::WHITE,
                                        background: Image::from_bytes(
                                            format!(
                                                "bytes://manga_image_{}_overlay_{}",
                                                page_id, index
                                            ),
                                            STANDARD.decode(back).ok()?,
                                        ),
                                    })
                                }
                            }

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
                            Some(Arc::new((
                                Image::from_bytes(
                                    format!("bytes://manga_image_{}", page_id),
                                    res.to_vec(),
                                ),
                                translations,
                            )))
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

///img_size, img_padding, scale
fn max(img: Vec2, area: Vec2) -> (Vec2, Vec2, f32) {
    let scale_x = area.x / img.x;
    let scale_y = area.y / img.y;
    let min_scale = scale_x.min(scale_y);
    let img_area = vec2(img.x * min_scale, img.y * min_scale);
    (img_area, area - img_area, min_scale)
}

fn get_img<'a>(
    page_id: &'a str,
    imgs: &'a mut ImageStorage,
) -> Arc<(Image<'static>, Vec<ReaderTranslationArea>)> {
    if let Some(img) = imgs.get(page_id) {
        if let Some(v) = img.req.task.ready() {
            match v {
                None => imgs.error.clone(),
                Some(v) => return v.clone(),
            }
        } else {
            imgs.loading.clone()
        }
    } else {
        unreachable!()
    }
}

fn render_overlay(items: &Vec<ReaderTranslationArea>, start: Vec2, scale: f32, ui: &mut Ui) {
    let my_put = |ui: &mut Ui, rect: Rect, layout: Layout, text| {
        ui.allocate_ui_at_rect(rect, |ui| ui.with_layout(layout, |ui| ui.add(text)).inner)
            .inner
    };
    let layout = generate_layout(Align2::CENTER_CENTER);
    for item in items {
        let min = vec2(item.min_x as f32, item.min_y as f32) * scale + start;
        let max = vec2(item.max_x as f32, item.max_y as f32) * scale + start;
        let rect = Rect::from_min_max(min.to_pos2(), max.to_pos2());
        item.background.paint_at(ui, rect);

        let text = item.get_size(rect, ui);
        TEXT_SHADOWS.iter().for_each(|offset| {
            let data = generate_label_data(text.clone(), rect, offset, Some(item.outline_color));
            my_put(ui, data.0, layout, data.1);
        });
        let data = generate_label_data(text, rect, &[0.0; 2], None);
        my_put(ui, data.0, layout, data.1);
    }
}

fn generate_label_data(
    mut widget_text: WidgetText,
    mut rect: Rect,
    offset: &[f32; 2],
    color: Option<Color32>,
) -> (Rect, Label) {
    if let Some(color) = color {
        if let WidgetText::LayoutJob(lj) = &mut widget_text {
            lj.sections.get_mut(0).unwrap().format.color = color;
        }
    }
    rect = rect.translate(Vec2::from(offset));
    (rect, Label::new(widget_text))
}

/// displays images
fn display_images(
    v: Arc<MangaReaderResponse>,
    hierachy: &[String],
    page_data: &mut PageData,
    progress: &Progress,
    ui: &mut Ui,
    rm: &ReadingMode,
    imgs: &mut ImageStorage,
    size: Vec2,
    view_start: Vec2,
) {
    let chapter = progress.chapter.clone();
    let ch = get_page_resp(v, hierachy, page_data, &chapter, ui.ctx());
    match rm {
        ReadingMode::Single => {
            let (r, img) = get_img_(imgs, ch, progress.image).unwrap();
            let (size, gaps, scale) = max(vec2(r.width as f32, r.height as f32), size);
            let start = (gaps * 0.5) + view_start;
            img.0
                .paint_at(ui, Rect::from_min_size(start.to_pos2(), size));
            render_overlay(&img.1, start, scale, ui)
        }
        ReadingMode::Double(lr) => {
            let (r1, mut img1) = get_img_(imgs, ch.clone(), progress.image).unwrap();
            if let Some((r2, mut img2)) = get_img_(imgs, ch, progress.image + 1) {
                let new_size = vec2(size.x / 2., size.y);
                let (size1, gaps1, scale1) = max(vec2(r1.width as f32, r1.height as f32), new_size);
                let (size2, gaps2, scale2) = max(vec2(r2.width as f32, r2.height as f32), new_size);
                let start1 = vec2(gaps1.x, gaps1.y / 2.) + view_start;
                let start2 = view_start + vec2(size.x / 2., gaps2.y / 2.);
                if !lr {
                    let temp = img1;
                    img1 = img2;
                    img2 = temp;
                }
                img1.0
                    .paint_at(ui, Rect::from_min_size(start1.to_pos2(), size1));
                img2.0
                    .paint_at(ui, Rect::from_min_size(start2.to_pos2(), size2));
                render_overlay(&img1.1, start1, scale1, ui);
                render_overlay(&img2.1, start2, scale2, ui)
            } else {
                let (size, gaps, scale) = max(vec2(r1.width as f32, r1.height as f32), size);
                let start = (gaps * 0.5) + view_start;
                img1.0
                    .paint_at(ui, Rect::from_min_size(start.to_pos2(), size));
                render_overlay(&img1.1, start, scale, ui)
            }
        }
        ReadingMode::Strip => unimplemented!(),
        ReadingMode::Row(_) => unimplemented!(),
    }
}

fn get_img_(
    imgs: &mut ImageStorage,
    ch: State,
    page: u32,
) -> Option<(
    ReaderPage,
    Arc<(Image<'static>, Vec<ReaderTranslationArea>)>,
)> {
    match ch {
        State::ReaderPageResponse(v_) => match v_.get_page(page) {
            Action::Page(r) => {
                let img = get_img(&r.page_id, imgs);
                Some((r.clone(), img))
            }
            _ => None,
        },
        State::ChapterLoading => Some((ReaderPage::new(100, 100), imgs.loading.clone())),
        State::ChapterError => Some((ReaderPage::new(100, 100), imgs.error.clone())),
        State::NoChapter => Some((ReaderPage::new(100, 100), imgs.error.clone())),
    }
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
    fn get_size(&self, ctx: &Context) -> Vec2 {
        #[cfg(target_arch = "wasm32")]
        let screen = get_window_dimensions();
        #[cfg(not(target_arch = "wasm32"))]
        let screen = ctx.input(|i| i.viewport().outer_rect).unwrap().size();
        screen
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

pub struct ReaderPageTranslation {
    translations: Vec<ReaderTranslationArea>,
}

pub struct ReaderTranslationArea {
    pub translated_text: HashMap<String, String>,
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
    pub text_color: Color32,
    pub outline_color: Color32,
    pub background: Image<'static>,
}

impl ReaderTranslationArea {
    fn get_size(&self, rect: Rect, ui: &mut Ui) -> WidgetText {
        let v = FontId::new(12., FontFamily::Name(Fonts::CcWildWords.to_string().into()));
        let text = match self.translated_text.get("eng") {
            Some(v) => v,
            None => self.translated_text.values().next().unwrap(),
        }
        .to_string();

        let mut lj = LayoutJob::simple(text, v.clone(), self.text_color, rect.width());
        let mut fontsize = 12;
        //TODO: cache fontsize
        //TODO: add guess based on rect size
        //TODO: add check for extreme height and small width
        //TODO: find a good algorithm to calculate fontsize
        let font_range = [5, 20];
        let mut increase = true;
        let update_fontsize = |size: u32, lj: &mut LayoutJob| {
            lj.sections[0].format = TextFormat::simple(
                FontId::new(size as f32, v.family.clone()),
                lj.sections[0].format.color,
            )
        };
        let y = rect.size().y;
        let mut last_action = LastAction::None;
        loop {
            if fontsize >= font_range[1] || fontsize <= font_range[0] {
                break;
            }
            if increase {
                fontsize += 1;
            } else {
                fontsize -= 1;
            }
            update_fontsize(fontsize, &mut lj);
            let size = ui.fonts(|fonts| fonts.layout_job(lj.clone())).size();
            if y < size.y {
                increase = false;
            } else {
                increase = true;
            }
            if last_action == LastAction::Increase && !increase {
                update_fontsize(fontsize - 1, &mut lj);
                break;
            } else if last_action == LastAction::Decrease && increase {
                update_fontsize(fontsize + 1, &mut lj);
                break;
            } else {
                last_action = LastAction::new(increase)
            }
        }
        WidgetText::from(lj)
    }
}

#[derive(PartialEq, Eq)]
enum LastAction {
    Decrease,
    Increase,
    None,
}

impl LastAction {
    fn new(increase: bool) -> Self {
        match increase {
            true => Self::Increase,
            false => Self::Decrease,
        }
    }
}

fn generate_layout(align: Align2) -> Layout {
    match align {
        Align2::LEFT_TOP => Layout::left_to_right(Align::LEFT).with_main_wrap(true),
        Align2::LEFT_CENTER => Layout {
            main_dir: Direction::BottomUp,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: true,
            cross_align: Align::LEFT,
            cross_justify: false,
        },
        Align2::LEFT_BOTTOM => Layout {
            main_dir: Direction::BottomUp,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: false,
            cross_align: Align::LEFT,
            cross_justify: false,
        },
        Align2::RIGHT_TOP => Layout::right_to_left(Align::TOP).with_main_wrap(true),
        Align2::RIGHT_CENTER => Layout::right_to_left(Align::Center).with_main_wrap(true),
        Align2::RIGHT_BOTTOM => Layout::right_to_left(Align::BOTTOM).with_main_wrap(true),
        Align2::CENTER_TOP => Layout {
            main_dir: Direction::TopDown,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: false,
            cross_align: Align::Center,
            cross_justify: false,
        },
        Align2::CENTER_CENTER => Layout {
            main_dir: Direction::TopDown,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: true,
            cross_align: Align::Center,
            cross_justify: false,
        },
        Align2::CENTER_BOTTOM => Layout {
            main_dir: Direction::BottomUp,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: false,
            cross_align: Align::Center,
            cross_justify: false,
        },
    }
}

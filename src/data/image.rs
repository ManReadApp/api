use crate::get_app_data;
use crate::widgets::image_overlay::ImageOverlay;
use api_structure::image::MangaCoverRequest;
use api_structure::search::Status;
use chrono::Duration;
use eframe::emath::vec2;
use egui::{Image, Sense};
use ethread::ThreadHandler;
use futures_util::{stream, StreamExt};
use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::future::Future;

#[derive(Default)]
pub struct CoverStorage {
    items: HashMap<String, CoverTimeStamp>,
}

impl CoverStorage {
    pub fn get_main(&self, manga_id: &str) -> Option<&ImageOverlay> {
        let item = self.items.get(manga_id)?;
        match item.image.task.ready()? {
            None => None,
            Some(v) => Some(v),
        }
    }
    fn new(data: HashMap<String, Option<ImageOverlay>>) -> Self {
        Self {
            items: data
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        CoverTimeStamp::new_manual(ThreadHandler::new(|| value)),
                    )
                })
                .collect(),
        }
    }

    pub fn append(&mut self, other: Self) {
        for other in other.items {
            self.items.insert(other.0, other.1);
        }
    }

    pub async fn download_many(ids: HashMap<String, (Status, String, u32)>) -> Self {
        let reqs = ids
            .into_iter()
            .map(|(manga_id, (status, ext, number))| Self::download(manga_id, status, ext));
        let v: HashMap<String, Option<ImageOverlay>> = stream::iter(reqs)
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .map(|v| (v.0, Some(v.1)))
            .collect::<HashMap<_, _>>();
        Self::new(v)
    }

    fn download(
        manga_id: String,
        status: Status,
        ext: String,
    ) -> impl Future<Output = Option<(String, ImageOverlay)>> + Sized {
        async move {
            let app = get_app_data();
            let token = format!("Bearer {}", app.get_access_token().await.unwrap());
            let bytes = app
                .client
                .post(app.url.join("cover").unwrap())
                .header(AUTHORIZATION, token)
                .json(&MangaCoverRequest {
                    manga_id: manga_id.clone(),
                    file_ext: ext,
                })
                .send()
                .await
                .ok()?
                .bytes()
                .await
                .ok()?;
            let img = Image::from_bytes(format!("cover://{}", manga_id), bytes.to_vec())
                .sense(Sense::click())
                .fit_to_exact_size(vec2(200., 300.));
            let overlay = match status {
                Status::Dropped => ImageOverlay::dropped(img),
                Status::Hiatus => ImageOverlay::hiatus(img),
                Status::Ongoing => ImageOverlay::ongoing(img),
                Status::Completed => ImageOverlay::completed(img),
                Status::Upcoming => ImageOverlay::upcoming(img),
            };

            Some((manga_id.clone(), overlay))
        }
    }
}

struct CoverTimeStamp {
    opened: Option<Duration>,
    image: ThreadHandler<Option<ImageOverlay>>,
}

impl CoverTimeStamp {
    fn new_manual(image: ThreadHandler<Option<ImageOverlay>>) -> Self {
        Self {
            opened: None,
            image,
        }
    }
}

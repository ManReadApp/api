#[tokio::test]
async fn hash() {
    let hasher = HasherConfig::new().to_hasher();
    let db = Arc::new(establish("data".into(), false).await.unwrap());
    let mangas: Vec<RecordData<Manga>> = Manga::all(&*db).await.unwrap();
    for manga in mangas {
        for chapter in manga.data.chapters {
            let thing = chapter.thing;
            let ch = thing.id().to_string();
            let v: RecordData<Chapter> = thing.get(&*db).await.unwrap().unwrap();
            for (key, value) in v.data.versions {
                let record: RecordData<ChapterVersion> =
                    value.thing.get(&*db).await.unwrap().unwrap();
                for page_id in record.data.pages {
                    let key = key.split_once(":").unwrap().1.to_string();
                    let thing = page_id.thing;
                    let thing_clone = ThingFunc::new(thing.0.clone());
                    let page: Page = thing.get(&*db).await.unwrap().unwrap();
                    if page.hash.is_none() {
                        let path = format!(
                            "data/mangas/{}/{}/{}/{}.{}",
                            manga.id.id().to_string(),
                            ch,
                            key,
                            page.page,
                            page.ext
                        );
                        if let Ok(image) = image::open(&path) {
                            let hash = hasher.hash_image(&image).to_base64();
                            let v: Value = thing_clone
                                .patch(&*db, PatchOp::add("hash", hash))
                                .await
                                .unwrap()
                                .unwrap();
                        } else {
                            println!("{}", path);
                        }
                    }
                }
            }
        }
    }
}

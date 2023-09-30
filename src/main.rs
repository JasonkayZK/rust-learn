use autosurgeon::{Hydrate, hydrate, Reconcile, reconcile};

#[derive(Default, Debug, Clone, Reconcile, Hydrate, PartialEq)]
pub struct Data {
    pub id: Option<i32>,
    pub content: String,
    pub content_preview: Option<String>,
    // data_type(文本=text、图片=image)
    pub data_type: String,
    pub md5: String,
    pub create_time: i32,
    pub is_favorite: i32,
    pub tags: String,
    pub latest_addr: String,
}

fn main() {
    let mut data1 = Data {
        id: Some(1),
        content: "123".to_string(),
        content_preview: None,
        data_type: "text".to_string(),
        md5: "55555".to_string(),
        create_time: 111110,
        is_favorite: 0,
        tags: "".to_string(),
        latest_addr: "".to_string(),
    };

    let mut doc = automerge::AutoCommit::new();
    reconcile(&mut doc, &data1).unwrap();

    // Fork and make changes
    let mut doc2 = doc.fork().with_actor(automerge::ActorId::random());
    let mut data2: Data = hydrate(&doc2).unwrap();
    data2.content = "abc".to_string();
    reconcile(&mut doc2, &data2).unwrap();

    // Concurrently on doc1
    data1.md5 = "def".to_string();
    reconcile(&mut doc, &data1).unwrap();

    // Now merge the documents
    doc.merge(&mut doc2).unwrap();

    let merged: Data = hydrate(&doc).unwrap();
    println!("merged: {:?}", merged);
    assert_eq!(merged, Data {
        id: Some(1),
        content: "abc".to_string(),
        content_preview: None,
        data_type: "text".to_string(),
        md5: "def".to_string(),
        create_time: 111110,
        is_favorite: 0,
        tags: "".to_string(),
        latest_addr: "".to_string(),
    })
}

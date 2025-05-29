use anyhow::Error;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::Value;
use std::sync::LazyLock;
use url::Url;

static ROOT_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://www.songsterr.com/").unwrap());
static ROOT_URL_SONG: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://www.songsterr.com/a/wsa").unwrap());

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    artist: String,
    song_id: u32,
    title: String,
    // has_chords: bool,
    has_player: bool,
}

impl Song {
    fn sanitize(s: &str) -> String {
        s.to_lowercase()
            .replace(|c: char| c.is_ascii_punctuation(), "")
            .replace(|c: char| c.is_whitespace(), "-")
    }
    pub fn link(&self) -> Url {
        let stem = format!(
            "{}-{}-{}-s{}",
            Self::sanitize(&self.artist),
            Self::sanitize(&self.title),
            if self.has_player { "tab" } else { "chords" },
            Self::sanitize(&self.song_id.to_string())
        );

        let mut url = ROOT_URL_SONG.clone();
        url.path_segments_mut().unwrap().push(&stem);

        url
    }
}

pub async fn search(query: &str) -> Result<Vec<Song>, Error> {
    let mut url = ROOT_URL.clone();
    url.query_pairs_mut().append_pair("pattern", query).finish();
    let resp = reqwest::get(url);
    let text = resp.await.unwrap().text().await.unwrap();
    let document = Html::parse_document(&text);
    let sel = Selector::parse("script[id=state]").unwrap();
    let list = if let Some(text) = document.select(&sel).next() {
        let to_deser = text.text().collect::<String>();
        let deser: Value = serde_json::from_str(&to_deser)?;
        let songs_list = &deser["songs"]["songs"]["list"];
        let songs_list = if let Value::Array(sl) = songs_list {
            let songs = sl
                .iter()
                .map(|item| serde_json::from_value(item.clone()).map_err(|e| Error::new(e)))
                .collect::<Result<Vec<Song>, Error>>();
            songs
        } else {
            Err(Error::msg("invalid format"))
        };

        songs_list
    } else {
        Err(Error::msg("invalid format"))
    };
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search() {
        let results = search("fountains of wayne").await.unwrap();
        assert!(!results.is_empty());
    }
}

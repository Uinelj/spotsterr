use anyhow::Error;
use futures_util::{Stream, StreamExt};
use rspotify::{
    ClientError,
    model::{FullTrack, PlayableItem, PlaylistId, PlaylistItem},
    prelude::BaseClient,
};
use tracing::info;

/// handles going from a PlaylistItem to a FullTrack.
fn handle_item(item: Result<PlaylistItem, ClientError>) -> Result<FullTrack, Error> {
    match item?.track.ok_or(Error::msg("Track not found"))? {
        PlayableItem::Track(t) => Ok(t),
        PlayableItem::Episode(_) => Err(Error::msg("Episode not supported")),
    }
}

pub(crate) async fn fetch_playlist<C: BaseClient>(
    client: &C,
    id: String,
) -> Result<impl Stream<Item = Result<FullTrack, Error>>, Error> {
    info!(msg = "fetching playlist", id = id);
    let pl_id = PlaylistId::from_id(id)?;
    let pl = client.playlist_items(pl_id, None, None);
    Ok(pl.map(|item| handle_item(item)))
}

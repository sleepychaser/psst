use crate::data::Playlist;
use druid::Data;
use std::sync::Arc;

#[derive(Clone, Debug, Data, Eq, PartialEq, Hash)]
pub enum Route {
    Home,
    SearchResults,
    AlbumDetail,
    ArtistDetail,
    PlaylistDetail,
    Library,
}

#[derive(Clone, Debug, Data)]
pub enum Navigation {
    Home,
    SearchResults(String),
    AlbumDetail(Arc<str>),
    ArtistDetail(Arc<str>),
    PlaylistDetail(Playlist),
    Library,
}

impl Navigation {
    pub fn as_route(&self) -> Route {
        match self {
            Navigation::Home => Route::Home,
            Navigation::SearchResults(_) => Route::SearchResults,
            Navigation::AlbumDetail(_) => Route::AlbumDetail,
            Navigation::ArtistDetail(_) => Route::ArtistDetail,
            Navigation::PlaylistDetail(_) => Route::PlaylistDetail,
            Navigation::Library => Route::Library,
        }
    }
}
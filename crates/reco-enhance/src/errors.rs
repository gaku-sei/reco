#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Image(#[from] image::ImageError),

    #[error(transparent)]
    Ort(#[from] ort::Error),

    #[error(transparent)]
    CbzReader(#[from] reco::ReaderOpenError),

    #[error(transparent)]
    CbzArchive(#[from] reco::CreateArchiveError),

    #[error(transparent)]
    LoadImage(#[from] reco::LoadImageError),

    #[error(transparent)]
    Insertion(#[from] reco::InsertionError),
}

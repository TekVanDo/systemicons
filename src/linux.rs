use resvg::usvg::TreeParsing;
use std::{fs, io};

use crate::Error;

lazy_static::lazy_static! {
    static ref SHARED_MIME_INFO: xdg_mime::SharedMimeInfo = xdg_mime::SharedMimeInfo::new();
}

pub fn get_icon(path: &str, size: u16) -> Result<Vec<u8>, Error> {
    for mime_type in SHARED_MIME_INFO.get_mime_types_from_file_name(path) {
        for icon_name in SHARED_MIME_INFO.lookup_icon_names(&mime_type) {
            //TODO: scale and theme
            match freedesktop_icons::lookup(&icon_name)
                .with_size(size)
                .with_theme("Cosmic")
                .with_cache()
                .find()
            {
                Some(path) => match path.extension().and_then(|x| x.to_str()) {
                    Some("png") => {
                        let data = fs::read(&path)?;
                        return Ok(data);
                    }
                    Some("svg") => {
                        let data = fs::read(&path)?;
                        let usvg =
                            resvg::usvg::Tree::from_data(&data, &resvg::usvg::Options::default())?;
                        let tree = resvg::Tree::from_usvg(&usvg);
                        let mut pixmap =
                            resvg::tiny_skia::Pixmap::new(size.into(), size.into()).unwrap();
                        tree.render(
                            resvg::tiny_skia::Transform::identity(),
                            &mut pixmap.as_mut(),
                        );
                        let png = pixmap.encode_png()?;
                        return Ok(png);
                    }
                    _ => {}
                },
                None => {}
            }
        }
    }
    Err(Error::from(io::Error::new(
        io::ErrorKind::NotFound,
        "icon not found",
    )))
}

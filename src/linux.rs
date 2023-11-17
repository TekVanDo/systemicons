use std::{fs, io};

use crate::{Error, Icon};

lazy_static::lazy_static! {
    static ref SHARED_MIME_INFO: xdg_mime::SharedMimeInfo = xdg_mime::SharedMimeInfo::new();
}

pub fn get_icon(path: &str, size: u16) -> Result<Icon, Error> {
    for mime_type in SHARED_MIME_INFO.get_mime_types_from_file_name(path) {
        for icon_name in SHARED_MIME_INFO.lookup_icon_names(&mime_type) {
            //TODO: scale and theme
            match freedesktop_icons::lookup(&icon_name)
                .with_size(size)
                .with_theme("Cosmic")
                .with_cache()
                .force_svg()
                .find()
            {
                Some(path) => match path.extension().and_then(|x| x.to_str()) {
                    Some("png") => {
                        let data = fs::read(&path)?;
                        return Ok(Icon::Png(data));
                    }
                    Some("svg") => {
                        let data = fs::read(&path)?;
                        return Ok(Icon::Svg(data));
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

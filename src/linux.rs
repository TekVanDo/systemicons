use resvg::usvg::TreeParsing;
use std::{
    convert::TryInto,
    ffi::OsStr,
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use crate::Error;

lazy_static::lazy_static! {
    static ref SHARED_MIME_INFO: xdg_mime::SharedMimeInfo = xdg_mime::SharedMimeInfo::new();
}

pub fn get_icon(ext: &str, size: i32) -> Result<Vec<u8>, Error> {
    let size: u16 = size
        .try_into()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    //TODO: take path instead of extension
    let file_name = format!("file.{}", ext);
    for mime_type in SHARED_MIME_INFO.get_mime_types_from_file_name(&file_name) {
        for icon_name in SHARED_MIME_INFO.lookup_icon_names(&mime_type) {
            //TODO: scale and theme
            println!("{icon_name}");
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

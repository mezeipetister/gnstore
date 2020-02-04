use crate::error::Error::*;
use crate::prelude::*;

pub fn check_email(address: &str) -> AppResult<()> {
    if !address.contains("@") {
        return Err(BadRequest(
            "Nem megfelelő email formátum. Hiányzó karakter: @.".into(),
        ));
    }
    if !address.contains(".") {
        return Err(BadRequest(
            "Nem megfelelő email formátum. Hiányzó karakter: (pont).".into(),
        ));
    }
    if address.len() < 4 {
        return Err(BadRequest(
            "Nem megfelelő email formátum. Túl rövid.".into(),
        ));
    }
    Ok(())
}

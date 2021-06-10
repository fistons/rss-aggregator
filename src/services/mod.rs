use std::sync::Arc;

use log::debug;

use crate::{DbPool, services};

pub mod channels;
pub mod items;
pub mod users;
pub mod auth;

pub fn refresh(pool: &Arc<DbPool>, user_id: i32) -> Result<(), diesel::result::Error> {
    let channels = crate::services::channels::select_all_by_user_id(pool, user_id)?;

    for channel in channels.iter() {
        refresh_chan(pool, channel.id, user_id)?;
    }
    Ok(())
}

pub fn refresh_chan(pool: &Arc<DbPool>, channel_id: i32, user_id: i32) -> Result<(), diesel::result::Error> {
    let channel = crate::services::channels::select_by_id_and_user_id(channel_id, user_id, pool)?;
    debug!("Fetching {}", &channel.name);

    let content = reqwest::blocking::get(&channel.url)
        .unwrap()
        .bytes()
        .unwrap();
    let rss_channel = rss::Channel::read_from(&content[..]).unwrap();
    for item in rss_channel.items.into_iter() {
        let i = crate::model::item::NewItem::from_rss_item(item, channel.id);
        services::items::insert(i, pool).unwrap();
    }

    Ok(())
}

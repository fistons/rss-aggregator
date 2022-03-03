use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use sea_orm::{entity::*, query::*};
use sea_orm::DatabaseConnection;

use entity::{channels, users};
use entity::channels::Entity as Channel;

use crate::errors::ApiError;

#[derive(Clone)]
pub struct ChannelService {}

impl ChannelService {
    pub fn new() -> Self { ChannelService {} }

    pub async fn select_by_id_and_user_id(
        &self,
        db: &DatabaseConnection,
        u_id: i32,
        chan_id: i32,
    ) -> Result<(), ApiError> {
        let result : Vec<_> = channels::Entity::find()
            
            .filter(channels::Column::Id.eq(chan_id))
            .filter(users::Column::Id.eq(u_id))
            .all(db)
            .await?;

        log::debug!("{:?}", result);
        Ok(())
    }


    // pub fn create_or_link_channel(
    //     &self,
    //     new_channel: NewChannel,
    //     other_user_id: i32,
    // ) -> Result<Channel, ApiError> {
    //     let connection = self.pool.get().unwrap();
    //     
    //     let chan = match channels
    //         .filter(url.eq(&new_channel.url))
    //         .first::<Channel>(&connection)
    //     {
    //         Ok(found) => found,
    //         Err(diesel::NotFound) => self.create_new_channel(&new_channel)?,
    //         Err(x) => return Err(x.into()),
    //     };
    //     
    //     let new_channel_user = ChannelUser{channel_id: chan.id, user_id: other_user_id};
    //     
    //     diesel::insert_into(channel_users)
    //         .values(new_channel_user)
    //         .on_conflict_do_nothing()
    //         .execute(&connection)?;
    // 
    //     Ok(chan)
    // }
    // 
    // fn create_new_channel(
    //     &self,
    //     new_channel: &NewChannel,
    // ) -> Result<Channel, ApiError> {
    //     let connection = self.pool.get().unwrap();
    // 
    //     let generated_id: i32 = diesel::insert_into(channels)
    //         .values(new_channel)
    //         .returning(id)
    //         .get_result(&connection)?;
    // 
    //     Ok(channels
    //         .filter(id.eq(generated_id))
    //         .first::<Channel>(&connection)?)
    // }
    // 
    // pub fn select_all_by_user_id(&self, u_id: i32) -> Result<Vec<Channel>, ApiError> {
    //     let channel_ids = channel_users
    //         .filter(user_id.eq(u_id))
    //         .select(channel_id);
    // 
    //     Ok(channels
    //         .filter(id.eq(any(channel_ids)))
    //         .load::<Channel>(&self.pool.get().unwrap())?)
    // }
    // 
    // pub fn select_all(&self) -> Result<Vec<Channel>, ApiError> {
    //     Ok(channels.load::<Channel>(&self.pool.get().unwrap())?)
    // }
    // 
    // pub fn select_by_id_and_user_id(
    //     &self,
    //     u_id: i32,
    //     chan_id: i32,
    // ) -> Result<Channel, ApiError> {
    //     let channel_ids = channel_users
    //         .filter(user_id.eq(u_id))
    //         .select(channel_id);
    // 
    //     Ok(channels
    //         .filter(id.eq(any(channel_ids)).and(id.eq(chan_id)))
    //         .first::<Channel>(&self.pool.get().unwrap())?)
    // }
}

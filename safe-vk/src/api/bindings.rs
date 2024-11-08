use crate::{
    api::{
        messages::AbstractionMessages, photos::AbstractionPhotos, users::AbstractionUsers,
        MethodBuilder,
    },
    extract::{Ctx, Update},
    responses::Message,
    Result,
};

pub trait CtxAbstraction: AbstractionMessages + AbstractionPhotos + AbstractionUsers {}

impl CtxAbstraction for MethodBuilder {}

impl Ctx<Message> {
    pub fn messages(&self) -> MethodBuilder {
        <MethodBuilder as AbstractionMessages>::new(
            self.request.clone(),
            Some(self.message.peer_id),
        )
    }

    pub fn photos(&self) -> MethodBuilder {
        <MethodBuilder as AbstractionPhotos>::new(self.request.clone(), Some(self.message.peer_id))
    }

    pub fn users(&self) -> MethodBuilder {
        <MethodBuilder as AbstractionUsers>::new(self.request.clone(), Some(self.message.peer_id))
    }
}

impl Ctx<Update> {
    pub fn messages(&self) -> Result<MethodBuilder> {
        let peer_id = self.find_peer_id(&self.object)?;
        Ok(<MethodBuilder as AbstractionMessages>::new(
            self.request.clone(),
            Some(peer_id),
        ))
    }

    pub fn photos(&self) -> Result<MethodBuilder> {
        let peer_id = self.find_peer_id(&self.object)?;
        Ok(<MethodBuilder as AbstractionPhotos>::new(
            self.request.clone(),
            Some(peer_id),
        ))
    }

    pub fn users(&self) -> Result<MethodBuilder> {
        let peer_id = self.find_peer_id(&self.object)?;
        Ok(<MethodBuilder as AbstractionUsers>::new(
            self.request.clone(),
            Some(peer_id),
        ))
    }
}

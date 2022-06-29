use sea_orm::ConnectionTrait;

pub mod entity;
pub mod user;

pub struct RdbRepository<'a, C: ConnectionTrait> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> RdbRepository<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }
}

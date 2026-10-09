use crate::compiler::Integration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LuaStyle<S> {
    Lua(String),
    LuaFile(Integration),
    Base(S),
}

impl<S: Clone> LuaStyle<S> {
    pub fn compile<T, D, R>(&self, _ast: &T, _data: &D) -> Result<R, S> {
        match self {
            LuaStyle::Lua(_) => todo!(),
            LuaStyle::LuaFile(_) => todo!(),
            LuaStyle::Base(err) => Err(err.clone()),
        }
    }
}

impl<S> From<S> for LuaStyle<S> {
    fn from(value: S) -> Self {
        Self::Base(value)
    }
}
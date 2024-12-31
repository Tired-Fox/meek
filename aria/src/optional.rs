use std::ops::{Deref, DerefMut};

use dioxus::prelude::{SuperFrom, SuperInto};

/// Mimics the builtin `Option` enum type
/// 
/// However, this type implements extra conversions and nice addons useful for this crates functionality.
#[derive(Default, strum::EnumIs)]
pub enum Optional<T> {
    Some(T),
    #[default]
    None,
}
impl<T> Optional<T> {
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => panic!("Optional::None cannot be unwrapped"),
        }
    }

    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => default,
        }
    }
    
    #[inline]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, provider: F) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => provider(),
        }
    }

    
    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Self::Some(val) => val,
            // SAFETY: the safety contract must be upheld by the caller.
            Self::None => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> Self {
        std::mem::replace(self, Optional::Some(value))
    }

    #[inline]
    pub fn map<F: FnOnce(T) -> R, R>(self, provider: F) -> Optional<R> {
        match self {
            Optional::None => Optional::None,
            Optional::Some(value) => Optional::Some(provider(value)),
        }
    }

    #[inline]
    pub fn map_or<F: FnOnce(T) -> R, R>(self, default: R, provider: F) -> Optional<R> {
        match self {
            Optional::None => Optional::Some(default),
            Optional::Some(value) => Optional::Some(provider(value)),
        }
    }
    
    #[inline]
    pub fn map_or_else<F: FnOnce(T) -> R, P: FnOnce() -> R, R>(
        self,
        default: P,
        provider: F,
    ) -> Optional<R> {
        match self {
            Optional::None => Optional::Some(default()),
            Optional::Some(value) => Optional::Some(provider(value)),
        }
    }

    #[inline]
    pub fn take(&mut self) -> Self {
        // FIXME replace `mem::replace` by `mem::take` when the latter is const ready
        std::mem::replace(self, Self::default())
    }

    #[inline]
    pub fn take_if<F: FnOnce(&mut Self) -> bool>(&mut self, predicate: F) -> Self {
        // FIXME replace `mem::replace` by `mem::take` when the latter is const ready
        if predicate(self) {
            std::mem::replace(self, Self::default())
        } else {
            Self::default()
        }
    }

    #[inline]
    pub fn filter<F: FnOnce(T) -> Option<T>>(self, predicate: F) -> Option<T> {
        match self {
            Self::None => None,
            Self::Some(value) => predicate(value),
        }
    }

    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, provider: F) -> Self {
        if let Optional::Some(value) = self.as_ref() {
            provider(&value)
        }
        self
    }

    #[inline]
    pub fn as_ref(&self) -> Optional<&T> {
        match self {
            Self::None => Optional::None,
            Self::Some(value) => Optional::Some(value),
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Optional<&mut T> {
        match self {
            Self::None => Optional::None,
            Self::Some(value) => Optional::Some(value),
        }
    }

    #[inline]
    pub fn or(self, other: T) -> Self {
        match self {
            Optional::Some(value) => Optional::Some(value),
            Optional::None => Optional::Some(other),
        }
    }

    #[inline]
    pub fn or_else<F: FnOnce() -> T>(self, provider: F) -> Self {
        match self {
            Optional::Some(value) => Optional::Some(value),
            Optional::None => Optional::Some(provider()),
        }
    }

    #[inline]
    pub fn xor(self, optb: Self) -> Self {
        match (self, optb) {
            (Self::Some(value), Self::None) => Optional::Some(value),
            (Self::None, Self::Some(value)) => Optional::Some(value),
            _ => Optional::None
        }
    }

    #[inline]
    pub fn and<R>(self, optb: Optional<R>) -> Optional<R> {
        match self {
            Self::None => Optional::None,
            Self::Some(_) => optb
        }
    }

    #[inline]
    pub fn and_then<R, F: FnOnce(T) -> Optional<R>>(self, provider: F) -> Optional<R> {
        match self {
            Self::None => Optional::None,
            Self::Some(value) => provider(value)
        }
    }

    #[inline]
    pub fn ok_or<E>(self, error: E) -> Result<T, E> {
        match self {
            Self::Some(value) => Ok(value),
            Self::None => Err(error)
        }
    }

    #[inline]
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, provider: F) -> Result<T, E> {
        match self {
            Self::Some(value) => Ok(value),
            Self::None => Err(provider())
        }
    }

    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        if let Self::None = self {
            *self = Optional::Some(value);
        }
        unsafe { self.as_mut().unwrap_unchecked() } 
    }

    #[inline]
    pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, provider: F) -> &mut T {
        if let Self::None = self {
            *self = Optional::Some(provider());
        }
        unsafe { self.as_mut().unwrap_unchecked() } 
    }

    #[inline]
    pub fn expect(self, msg: impl std::fmt::Display) -> T {
        match self {
            Self::Some(val) => val,
            Self::None => panic!("{msg}"),
        }
    }

    #[inline]
    pub fn insert(&mut self, value: T) -> &mut T {
        *self = Optional::Some(value);
        // SAFETY: the code above just filled the option
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn is_some_and<F: FnOnce(T) -> bool>(self, provider: F) -> bool {
        match self {
            Self::Some(value) => provider(value),
            Self::None => false
        }
    }
}

impl<T: Deref> Optional<T> {
    pub fn as_deref(&self) -> Optional<&T::Target> {
        match self {
            Self::None => Optional::None,
            Self::Some(value) => Optional::Some(&*value),
        }
    }
}

impl<T: DerefMut> Optional<T> {
    pub fn as_deref_mut(&mut self) -> Optional<&mut T::Target> {
        match self {
            Self::None => Optional::None,
            Self::Some(value) => Optional::Some(&mut *value),
        }
    }
}

impl<T: Default> Optional<T> {
    pub fn unwrap_or_default(self) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => T::default(),
        }
    }
}

impl<T: Clone> Clone for Optional<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Some(value) => Optional::Some(value.clone()),
            Self::None => Optional::None,
        }
    }
}

impl<T: PartialEq> PartialEq for Optional<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Some(current), Self::Some(other)) => current.eq(other),
            _ => false,
        }
    }
}

impl<T> Optional<T> {
    pub fn as_option(self) -> Option<T> {
        match self {
            Self::Some(value) => Some(value),
            Self::None => None,
        }
    }
}

impl<T: Into<U>, U> From<Option<T>> for Optional<U> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Optional::Some(value.into()),
            None => Optional::None,
        } 
    }
}

pub struct OptionalFromValue;
impl<T: Into<U>, U> SuperFrom<T, OptionalFromValue> for Optional<U> {
    fn super_from(value: T) -> Self {
        Optional::Some(Into::<U>::into(value)) 
    }
}
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Role {
    NotVerified = 0,
    User = 1,
    Author = 2,
    Moderator = 3,
    CoAdmin = 4,
    Admin = 5,
}

/// NotVerified will be used to reset the password
pub struct Kind {
    pub single: bool,
    pub kind: Role,
}
impl Kind {
    pub fn new(single: bool, kind: Role) -> Self {
        Self { single, kind }
    }
}

impl From<u32> for Kind {
    fn from(value: u32) -> Self {
        let s = value.to_string();
        let (role, single) = if s.len() == 1 {
            (Role::NotVerified, s.chars().next().unwrap())
        } else {
            assert_eq!(s.len(), 2);
            let mut chars = s.chars();
            let d: u32 = chars.next().unwrap().to_string().parse().unwrap();
            (Role::from(d), chars.next().unwrap())
        };

        let single = matches!(single, '1');
        Self { single, kind: role }
    }
}

impl From<Kind> for u32 {
    fn from(value: Kind) -> Self {
        let f = (value.kind as u8).to_string();
        let s = (value.single as u8).to_string();
        format!("{f}{s}").parse().unwrap()
    }
}

impl From<u32> for Role {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::User,
            2 => Self::Author,
            3 => Self::Moderator,
            4 => Self::CoAdmin,
            5 => Self::Admin,
            _ => Self::NotVerified,
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::NotVerified => write!(f, "Undifined"),
            Role::User => write!(f, "User"),
            Role::Moderator => write!(f, "Moderator"),
            Role::CoAdmin => write!(f, "Co-Admin"),
            Role::Admin => write!(f, "Admin"),
            Role::Author => write!(f, "Author"),
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        Ok(match role {
            "Author" => Self::Author,
            "Admin" => Self::Admin,
            "Co-Admin" => Self::CoAdmin,
            "Moderator" => Self::Moderator,
            "User" => Self::User,
            _ => Self::NotVerified,
        })
    }
}

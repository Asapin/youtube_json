use serde::Deserialize;
use vec1::Vec1;
use super::generic_types::{Image, SimpleText, AuthorBadge};

#[derive(Debug)]
pub struct ParticipantsList {
    participants: Vec1<Participant>,
}

impl<'de> Deserialize<'de> for ParticipantsList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        #[derive(Deserialize)]
        struct Outer {
            #[serde(rename = "liveChatParticipantsListRenderer")]
            inner: Inner,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Inner {
            participants: Vec1<Participant>,
        }

        let outer = Outer::deserialize(deserializer)?;
        Ok(ParticipantsList {
            participants: outer.inner.participants
        })
    }
}

#[derive(Debug)]
pub struct Participant {
    author_name: SimpleText,
    author_photo: Image,
    author_badges: Vec1<AuthorBadge>,
}

impl<'de> Deserialize<'de> for Participant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        #[derive(Deserialize)]
        struct Outer {
            #[serde(rename = "liveChatParticipantRenderer")]
            inner: Inner,
        }

        #[derive(Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Inner {
            author_name: SimpleText,
            author_photo: Image,
            author_badges: Vec1<AuthorBadge>,
        }

        let outer = Outer::deserialize(deserializer)?;
        Ok(Participant {
            author_name: outer.inner.author_name,
            author_photo: outer.inner.author_photo,
            author_badges: outer.inner.author_badges
        })
    }
}
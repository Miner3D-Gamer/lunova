#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// An "ID", an identifier that is not allowed to be same for two users
pub struct UserID {
    /// The id
    pub id: String,
}
impl UserID {
    /// Create a new id if the given string id is valid
    ///
    /// # Errors
    /// See [`IDConstraints`]
    pub fn new(
        id: String,
        condition: &IDConstraints,
    ) -> Result<Self, InvalidIdReason> {
        condition.is_valid_id(&id)?;
        Ok(Self {
            id,
        })
    }
    #[must_use]
    /// Get the underlying ID
    pub const fn get_id(&self) -> &String {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// The conditions an id must follow to be valid
pub struct IDConstraints {
    allowed: String,
    allowed_special: String,
    min_length: usize,
    max_length: usize,
    allow_special_character_at_start: bool,
    allow_special_character_at_end: bool,
}
impl core::default::Default for IDConstraints {
    fn default() -> Self {
        Self {
            allowed: "qwertzuioplkjhgfdsayxcvbnm".to_string(),
            allowed_special: "_-".to_string(),
            min_length: 4,
            max_length: 12,
            allow_special_character_at_end: false,
            allow_special_character_at_start: false,
        }
    }
}
impl IDConstraints {
    /// Check if an id is valid
    ///
    /// # Errors
    /// It can be invalid under these conditions:
    /// - Too short
    /// - Too long
    /// - Invalid character used
    /// - 2 special characters have been used directly next to each other
    /// - Special character is used at start or end
    pub fn is_valid_id(&self, id: &str) -> Result<(), InvalidIdReason> {
        if id.len() < self.min_length {
            return Err(InvalidIdReason::TooShort {
                given: id.len(),
                min: self.min_length,
            });
        }
        if id.len() > self.max_length {
            return Err(InvalidIdReason::TooLong {
                given: id.len(),
                max: self.max_length,
            });
        }
        let mut prev_special = None;
        let chars = id.chars();

        if !self.allow_special_character_at_start
            || !self.allow_special_character_at_end
        {
            let mut character_test_special_at_bound = chars.clone();
            if !self.allow_special_character_at_start
                && let Some(c) = character_test_special_at_bound.next()
                && self.allowed_special.contains(c)
            {
                return Err(InvalidIdReason::SpecialCharacterAtBoundOfID {
                    front: true,
                    char: c,
                });
            }
            if !self.allow_special_character_at_end
                && let Some(c) = character_test_special_at_bound.last()
                && self.allowed_special.contains(c)
            {
                return Err(InvalidIdReason::SpecialCharacterAtBoundOfID {
                    front: false,
                    char: c,
                });
            }
        }
        for (idx, c) in chars.enumerate() {
            if self.allowed.contains(c) {
                prev_special = None;
            } else if self.allowed_special.contains(c) {
                if let Some(previous_special) = prev_special {
                    return Err(
                        InvalidIdReason::SpecialCharacterNextToEachOther {
                            pos: idx,
                            first_char: previous_special,
                            second_char: c,
                        },
                    );
                }
                prev_special = Some(c);
            } else {
                return Err(InvalidIdReason::InvalidCharacter {
                    pos: idx,
                    char: c,
                });
            }
        }

        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// The reason an id is not valid
pub enum InvalidIdReason {
    /// Username is too short
    TooShort {
        /// How many characters the tried id had
        given: usize,
        /// The minimum amount of characters it needs
        min: usize,
    },
    /// Username is too long
    TooLong {
        /// How many characters the tried id had
        given: usize,
        /// The maximum amount of characters allowed
        max: usize,
    },
    /// Two special characters are next to each other
    SpecialCharacterNextToEachOther {
        /// Where the 2 characters collid
        pos: usize,
        /// The first special char
        first_char: char,
        /// The second special char
        second_char: char,
    },
    /// An invalid character is used
    InvalidCharacter {
        /// The position where the invalid character is
        pos: usize,
        /// What invalid character was found
        char: char,
    },
    /// When a special character was used at the front or back of the ID
    SpecialCharacterAtBoundOfID {
        /// If the char was at the front, otherwise the back
        front: bool,
        /// What character
        char: char,
    },
}

pub struct UserProfile {
    username: String,
    email: String,
    preferences: UserPreferences,
}

pub struct UserPreferences {
    theme: Theme,
    notifications_enabled: bool,
    auto_sync: bool,
}

pub enum Theme {
    Light,
    Dark,
    System,
}

impl UserProfile {
    pub fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            preferences: UserPreferences {
                theme: Theme::System,
                notifications_enabled: true,
                auto_sync: false,
            },
        }
    }

    pub fn update_preferences(&mut self, preferences: UserPreferences) {
        self.preferences = preferences;
    }
}
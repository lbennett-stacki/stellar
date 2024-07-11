pub struct PasswordGenerator {
    generator: passwords::PasswordGenerator,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            generator: passwords::PasswordGenerator {
                length: 16,
                numbers: true,
                lowercase_letters: true,
                uppercase_letters: true,
                symbols: true,
                spaces: false,
                exclude_similar_characters: false,
                strict: true,
            },
        }
    }
}

impl PasswordGenerator {
    pub fn generate_default() -> String {
        let generator = PasswordGenerator::default();

        generator.generate()
    }

    pub fn generate(&self) -> String {
        self.generator.generate_one().unwrap()
    }
}

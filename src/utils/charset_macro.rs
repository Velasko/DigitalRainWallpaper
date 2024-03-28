macro_rules! make_subset {
    ( $x:ident, $chars:expr ) => {
        pub struct $x(char);

        impl CharacterSubset for $x {
            fn parse(value: char) -> Option<Self> {
                if $chars.contains(value) {
                    Some(Self(value))
                } else {
                    None
                }
            }

            fn get_charset() -> Vec<char> {
                $chars.chars().collect::<Vec<char>>()
            }

            fn as_char(&self) -> &char {
                &self.0
            }
        }
    };
}
pub(super) use make_subset;

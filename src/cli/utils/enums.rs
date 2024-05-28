pub(crate) enum HelperMessage {
    TutorialWelcome,
    TutorialLocation,
    TutorialEditor,
}

impl std::fmt::Display for HelperMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelperMessage::TutorialWelcome => write!(
                f,
                r#"
--Welcome to Journey!--

This command-line interface app is here to help you document your thoughts,
experiences, and ideas effortlessly.  Let's get you started :)
"#
            ),
            HelperMessage::TutorialLocation => write!(
                f,
                r#"
--Set your usual location--

Your journal will use your default location to automatically detect your
default time zone and to detect the current weather.  This will also be printed
in your entries.  To ensure the best results, make sure that the last part of
your location is somewhere that is specific enough for accurate time zone and
weather data.

Examples:
  Avenida 9 SO - Carchi, Guayaquil
  1600 Pennsylvania Avenue NW, Washington, D.C
  Lor Marzuki, Singapore City
  Al Quds Open University, Gaza
  25 Paddington Grn, City of Westminster
"#
            ),
            HelperMessage::TutorialEditor => write!(
                f,
                r#"
--Set your editor--

Journey lets you use your preferred text editor, such as vim, nano, or emacs.
"#
            ),
        }
    }
}

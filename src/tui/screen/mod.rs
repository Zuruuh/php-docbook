use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Paragraph, Widget},
};

#[derive(Default, Debug)]
pub struct HomeScreen;

/// Original Art by Donovan Bake
/// https://www.asciiart.eu/books/books
const BOOK_ASCII: &str = indoc::indoc! {r#"
     __...--~~~~~-._   _.-~~~~~--...__
   //    THE PHP    `V'    DOCBOOK    \\
  //                 |                 \\
 //__...--~~~~~~-._  |  _.-~~~~~~--...__\\
//__.....----~~~~._\ | /_.~~~~----.....__\\
\===================\\|//=================/
"#};

impl Widget for HomeScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(BOOK_ASCII).centered().render(area, buf);
    }
}

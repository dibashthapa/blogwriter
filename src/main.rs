use std::{fs, io::{Cursor, Error}};
mod cmd;

use pulldown_cmark::{Event, Tag, HeadingLevel, html};

#[derive(Debug)]
struct TocEntry {
    slug:String,
    text:String
}

type Toc = Vec<TocEntry>;

fn main() -> Result<() , Error> {
    let contents = fs::read_to_string("file.md")?;

    let output = process_markdown_to_html(contents);
    println!("{}", output);
    Ok(())
}


fn process_markdown_to_html(markdown:String) -> String {
    let parser = pulldown_cmark::Parser::new(&markdown);
    
    let mut toc: Toc = Vec::new();
    let mut output: Vec<u8> = Vec::new();

    struct Heading {
        level: HeadingLevel,
        plain_text: String
    }

    let mut current_heading: Option<Heading> = None;

    let stream = parser.map(|event| {
        match &event {
            Event::Start(Tag::Heading(heading_level,_ ,_ )) => {
                current_heading = Some(Heading {
                    // TODO: Need to understand why this needs
                    // deferencing
                    level:*heading_level, 
                    plain_text: "".into()
                });
                return Event::Text("".into());       
            }, 
            Event::End(Tag::Heading(_, _,_ )) => {
                if let Some(heading) = current_heading.take() {
                    let tag = match heading.level {
                        HeadingLevel::H1 => "h1",
                        HeadingLevel::H2 => "h2",
                        HeadingLevel::H3 => "h3",
                        HeadingLevel::H4 => "h4",
                        HeadingLevel::H5 => "h5",
                        HeadingLevel::H6 => "h6",
                    };
                    let anchor = slugify(&heading.plain_text);
                    let header = heading.plain_text.clone();
                    let toc_entry = TocEntry {
                        text: heading.plain_text,
                        slug: anchor.clone()
                    };

                    toc.push(toc_entry);
                    return Event::Html(format!(
                    r#"
            <{tag} class="heading">
                    {header}
            </{tag}> 
                    "#).into())

                }
            },
            Event::Text(text) => {
                if let Some(current) = current_heading.as_mut() {
                    current.plain_text.push_str(text);
                    return Event::Text("".into())
                }
            }
            _ => {}
        }
        event
    });
  html::write_html(Cursor::new(&mut output), stream).unwrap();
  toc.into_iter().for_each(|item| {
      let title = item.text;
      let link = item.slug;
      println!(
        r#"
            <li class="item">
                <a href='#{link}'>
                    {title}
                </a>
            </li> 
        "#);
  });


  match String::from_utf8(output) {
    Ok(html) => html,
    Err(e) => e.to_string(),
  }
}

fn slugify(input:&str) -> String {
    let slug = input.to_ascii_lowercase().replace(" ","_");
    return slug;
}


use crate::rdocx_decl::{
    Cash, ToRdocx,
    list_level::{ListLevel, base_list_style_name},
    run::{self, Run},
    utils::{Color, Length, SectionBreak, SectionPageSize, border::Borders, tab::TabStop},
};
use rdocx::Document;
use serde::{Deserialize, Serialize};

//todo: This element is taken from rdocx; later, it needs to be changed in rdocx itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineSpacing {
    Length(Length),
    Multiple(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParagraphListLevelStyle {
    Name(String),
    Base,
    Style(Vec<ListLevel>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphListLevel {
    stile: ParagraphListLevelStyle,
    level: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Style {
    pub alignment: Option<Alignment>,
    pub space_before: Option<Length>,
    pub space_after: Option<Length>,
    pub indent_left: Option<Length>,
    pub indent_right: Option<Length>,
    pub signed_first_line_indent: Option<Length>,
    pub keep_with_next: Option<bool>,
    pub keep_together: Option<bool>,
    pub page_break_before: Option<bool>,
    pub widow_control: Option<bool>,
    pub line_spacing: Option<LineSpacing>,
    pub shading: Option<Color>,
    pub borders: Option<Borders>,
    pub tab_stops: Option<Vec<TabStop>>,
    pub outline_level: Option<u32>,
    pub section_break: Option<SectionBreak>,
    pub section_page_size: Option<SectionPageSize>,
    pub list_level: Option<ParagraphListLevel>,

    pub base_run: Option<run::Style>,
}

to_rdocx_static_dispatch! {
    {#[derive(Debug, Clone, Serialize, Deserialize)]},
    impl{'p},
    <{D: rdocx::Paragraph<'p>}, {T: &Option<run::Style>}>,
    Content {
        Run
    }
}

/// todo: add equation, footnote_ref, footnote, run_inheriting_mark, line_break, tab, picture, hyperlink, hyperlink_with_tooltip, style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub contents: Vec<Content>,
    pub style: Style,
}

impl ToRdocx<Document> for Paragraph {
    fn to_rdocx(&self, doc: &mut Document, _data: (), cash: &mut Cash) {
        let list_level = self.style.list_level.as_ref().map(|list_level| {
            (
                match &list_level.stile {
                    ParagraphListLevelStyle::Name(name) => cash
                        .lists
                        .get(name)
                        .cloned()
                        .unwrap_or(cash.lists.get(&base_list_style_name()).cloned().unwrap()),
                    ParagraphListLevelStyle::Base => {
                        cash.lists.get(&base_list_style_name()).cloned().unwrap()
                    }
                    ParagraphListLevelStyle::Style(list_level) => doc.add_list_definition(
                        &list_level
                            .iter()
                            .cloned()
                            .map(Into::into)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    ),
                },
                list_level.level,
            )
        });

        let mut paragraph = doc.add_paragraph("");
        self.contents.iter().for_each(|content| {
            content.to_rdocx(&mut paragraph, &self.style.base_run, cash);
        });

        paragraph.set_alignment_value(self.style.alignment.clone().map(Into::into));
        paragraph.set_space_before_value(self.style.space_before.clone().map(Into::into));
        paragraph.set_space_after_value(self.style.space_after.clone().map(Into::into));
        paragraph.set_indent_left_value(self.style.indent_left.clone().map(Into::into));
        paragraph.set_indent_right_value(self.style.indent_right.clone().map(Into::into));
        paragraph.set_signed_first_line_indent_value(
            self.style.signed_first_line_indent.clone().map(Into::into),
        );
        paragraph.set_keep_with_next_value(self.style.keep_with_next);
        paragraph.set_keep_together_value(self.style.keep_together);
        paragraph.set_page_break_before_value(self.style.page_break_before);
        paragraph.set_widow_control_value(self.style.widow_control);
        match self.style.line_spacing {
            Some(LineSpacing::Length(length)) => {
                paragraph.set_line_spacing(length.into_rdocx().to_pt())
            }
            Some(LineSpacing::Multiple(multiple)) => paragraph.set_line_spacing_multiple(multiple),
            None => {}
        }
        if let Some(shading) = &self.style.shading {
            paragraph.set_shading(&shading.to_string());
        }
        match &self.style.borders {
            Some(Borders::All { style, size, color }) => paragraph.set_border_all(
                style.clone().into(),
                size.into_rdocx().to_pt().floor() as u32,
                &color.to_string(),
            ),
            Some(Borders::Bottom(border)) => paragraph.set_border_bottom_with_space(
                border.style.into(),
                border.size.into_rdocx().to_pt().floor() as u32,
                border.space_points,
                &border.color.to_string(),
            ),
            None => {}
        }
        self.style
            .tab_stops
            .iter()
            .flatten()
            .cloned()
            .for_each(|tab_stop| match tab_stop {
                TabStop::TabStop {
                    alignment,
                    position,
                } => paragraph.set_add_tab_stop(alignment.into(), position.into()),
                TabStop::TabStopWithLeader {
                    alignment,
                    position,
                    leader,
                } => paragraph.set_add_tab_stop_with_leader(
                    alignment.into(),
                    position.into(),
                    leader.into(),
                ),
            });

        if let Some(outline_level) = self.style.outline_level {
            paragraph.set_outline_level(outline_level);
        }

        if let Some(section_break) = self.style.section_break {
            paragraph.set_section_break(section_break.into());
        }
        match self.style.section_page_size {
            Some(SectionPageSize::Landscape) => paragraph.set_section_landscape(),
            Some(SectionPageSize::Portrait) => paragraph.set_section_portrait(),
            Some(SectionPageSize::Size { width, height }) => {
                paragraph.set_section_page_size(width.into(), height.into())
            }
            None => {}
        }

        //todo: Add error handling.
        paragraph.set_numbering_value(list_level);
    }
}

impl Into<rdocx::Alignment> for Alignment {
    fn into(self) -> rdocx::Alignment {
        match self {
            Alignment::Left => rdocx::Alignment::Left,
            Alignment::Center => rdocx::Alignment::Center,
            Alignment::Right => rdocx::Alignment::Right,
            Alignment::Justify => rdocx::Alignment::Justify,
        }
    }
}

impl Style {
    pub fn alignment(mut self, alignment: Option<Alignment>) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn space_before(mut self, space_before: Option<Length>) -> Self {
        self.space_before = space_before;
        self
    }

    pub fn space_after(mut self, space_after: Option<Length>) -> Self {
        self.space_after = space_after;
        self
    }

    pub fn indent_left(mut self, indent_left: Option<Length>) -> Self {
        self.indent_left = indent_left;
        self
    }

    pub fn indent_right(mut self, indent_right: Option<Length>) -> Self {
        self.indent_right = indent_right;
        self
    }

    pub fn signed_first_line_indent(mut self, signed_first_line_indent: Option<Length>) -> Self {
        self.signed_first_line_indent = signed_first_line_indent;
        self
    }

    pub fn keep_with_next(mut self, keep_with_next: Option<bool>) -> Self {
        self.keep_with_next = keep_with_next;
        self
    }

    pub fn keep_together(mut self, keep_together: Option<bool>) -> Self {
        self.keep_together = keep_together;
        self
    }

    pub fn page_break_before(mut self, page_break_before: Option<bool>) -> Self {
        self.page_break_before = page_break_before;
        self
    }

    pub fn widow_control(mut self, widow_control: Option<bool>) -> Self {
        self.widow_control = widow_control;
        self
    }

    pub fn line_spacing(mut self, line_spacing: Option<LineSpacing>) -> Self {
        self.line_spacing = line_spacing;
        self
    }

    pub fn shading(mut self, shading: Option<Color>) -> Self {
        self.shading = shading;
        self
    }

    pub fn borders(mut self, borders: Option<Borders>) -> Self {
        self.borders = borders;
        self
    }

    pub fn tab_stops(mut self, tab_stops: Option<Vec<TabStop>>) -> Self {
        self.tab_stops = tab_stops;
        self
    }

    pub fn outline_level(mut self, outline_level: Option<u32>) -> Self {
        self.outline_level = outline_level;
        self
    }

    pub fn section_break(mut self, section_break: Option<SectionBreak>) -> Self {
        self.section_break = section_break;
        self
    }

    pub fn section_page_size(mut self, section_page_size: Option<SectionPageSize>) -> Self {
        self.section_page_size = section_page_size;
        self
    }

    pub fn base_run(mut self, base_run: Option<run::Style>) -> Self {
        self.base_run = base_run;
        self
    }
}

use crate::rdocx_decl::{
    ToRdocx,
    utils::{Color, Length},
};
use rdocx::Paragraph;
use serde::{Deserialize, Serialize};
use tracing::debug;

//todo: This element is taken from rdocs; later, it needs to be changed in rdocs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnderlineStyle {
    None,
    Single,
    Double,
    Thick,
    Dotted,
    Dash,
    Wave,
    Words,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Style {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline_style: Option<UnderlineStyle>,
    pub size: Option<Length>,
    pub font_name: Option<String>,
    pub language: Option<String>,
    pub color: Option<Color>,
    pub highlight: Option<Color>,
    pub shading: Option<Color>,
    pub strike: Option<bool>,
    pub double_strike: Option<bool>,
    pub all_caps: Option<bool>,
    pub small_caps: Option<bool>,
    pub superscript: Option<bool>,
    pub subscript: Option<bool>,
    pub character_spacing: Option<Length>,
    pub width_scale: Option<u32>,
    pub position: Option<i32>,
    pub hidden: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    Text(String),
}

/// todo: add tab, break, picture, field, symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub contents: Vec<Content>,
    pub style: Style,
}

impl<'r> ToRdocx<Paragraph<'r>, &Option<Style>> for Run {
    fn to_rdocx(
        &self,
        paragraph: &mut Paragraph<'r>,
        base_style: &Option<Style>,
        _cash: &mut super::Cash,
    ) {
        let style = base_style.clone()
            .map(|base_style| self.style.clone().and(base_style))
            .unwrap_or(self.style.clone());

        let mut run = paragraph.add_run("");
        self.contents.iter().for_each(|content| match content {
            Content::Text(text) => run.add_text(text),
        });
        run.set_bold_value(style.bold);
        run.set_italic_value(style.italic);
        run.set_underline_style_value(style.underline_style.map(Into::into));
        run.set_size_value(style.size.map(Length::into_rdocx).map(rdocx::Length::to_pt));
        run.set_font_value(style.font_name.as_ref().map(String::as_str));
        run.set_language_value(style.language.as_ref().map(String::as_str));
        run.set_color_value(
            style
                .color
                .as_ref()
                .map(ToString::to_string)
                .as_ref()
                .map(String::as_str),
        );
        run.set_highlight_value(
            style
                .highlight
                .as_ref()
                .map(ToString::to_string)
                .as_ref()
                .map(String::as_str),
        );
        run.set_shading_value(
            style
                .shading
                .as_ref()
                .map(ToString::to_string)
                .as_ref()
                .map(String::as_str),
        );
        run.set_strike_value(style.strike);

        if let Some(double_strike) = style.double_strike {
            run.set_double_strike(double_strike);
        }

        if let Some(all_caps) = style.all_caps {
            run.set_all_caps(all_caps);
        }

        if let Some(small_caps) = style.small_caps {
            run.set_small_caps(small_caps);
        }

        if style.superscript == Some(true) {
            run.set_superscript();
        }

        if style.subscript == Some(true) {
            run.set_subscript();
        }

        if let Some(character_spacing) = style.character_spacing {
            run.set_character_spacing(character_spacing.into());
        }

        if let Some(width_scale) = style.width_scale {
            run.set_width_scale(width_scale);
        }

        if let Some(position) = style.position {
            run.set_position(position);
        }

        if let Some(hidden) = style.hidden {
            run.set_hidden(hidden);
        }
    }
}

impl Into<rdocx::UnderlineStyle> for UnderlineStyle {
    fn into(self) -> rdocx::UnderlineStyle {
        match self {
            UnderlineStyle::None => rdocx::UnderlineStyle::None,
            UnderlineStyle::Single => rdocx::UnderlineStyle::Single,
            UnderlineStyle::Double => rdocx::UnderlineStyle::Double,
            UnderlineStyle::Thick => rdocx::UnderlineStyle::Thick,
            UnderlineStyle::Dotted => rdocx::UnderlineStyle::Dotted,
            UnderlineStyle::Dash => rdocx::UnderlineStyle::Dash,
            UnderlineStyle::Wave => rdocx::UnderlineStyle::Wave,
            UnderlineStyle::Words => rdocx::UnderlineStyle::Words,
        }
    }
}

impl Style {
    pub fn and(mut self, base: Self) -> Self {
        self.bold = self.bold.or(base.bold);
        self.italic = self.italic.or(base.italic);
        self.underline_style = self.underline_style.or(base.underline_style);
        self.size = self.size.or(base.size);
        self.font_name = self.font_name.or(base.font_name);
        self.language = self.language.or(base.language);
        self.color = self.color.or(base.color);
        self.highlight = self.highlight.or(base.highlight);
        self.shading = self.shading.or(base.shading);
        self.strike = self.strike.or(base.strike);
        self.double_strike = self.double_strike.or(base.double_strike);
        self.all_caps = self.all_caps.or(base.all_caps);
        self.small_caps = self.small_caps.or(base.small_caps);
        self.superscript = self.superscript.or(base.superscript);
        self.subscript = self.subscript.or(base.subscript);
        self.character_spacing = self.character_spacing.or(base.character_spacing);
        self.width_scale = self.width_scale.or(base.width_scale);
        self.position = self.position.or(base.position);
        self.hidden = self.hidden.or(base.hidden);

        self
    }

    pub fn bold(mut self, bold: Option<bool>) -> Self {
        self.bold = bold;
        self
    }

    pub fn italic(mut self, italic: Option<bool>) -> Self {
        self.italic = italic;
        self
    }

    pub fn underline_style(mut self, underline_style: Option<UnderlineStyle>) -> Self {
        self.underline_style = underline_style;
        self
    }

    pub fn size(mut self, size: Option<Length>) -> Self {
        self.size = size;
        self
    }

    pub fn font_name(mut self, font_name: Option<String>) -> Self {
        self.font_name = font_name;
        self
    }

    pub fn language(mut self, language: Option<String>) -> Self {
        self.language = language;
        self
    }

    pub fn color(mut self, color: Option<Color>) -> Self {
        self.color = color;
        self
    }

    pub fn highlight(mut self, highlight: Option<Color>) -> Self {
        self.highlight = highlight;
        self
    }

    pub fn shading(mut self, shading: Option<Color>) -> Self {
        self.shading = shading;
        self
    }

    pub fn strike(mut self, strike: Option<bool>) -> Self {
        self.strike = strike;
        self
    }

    pub fn double_strike(mut self, double_strike: Option<bool>) -> Self {
        self.double_strike = double_strike;
        self
    }

    pub fn all_caps(mut self, all_caps: Option<bool>) -> Self {
        self.all_caps = all_caps;
        self
    }

    pub fn small_caps(mut self, small_caps: Option<bool>) -> Self {
        self.small_caps = small_caps;
        self
    }

    pub fn superscript(mut self, superscript: Option<bool>) -> Self {
        self.superscript = superscript;
        self
    }

    pub fn subscript(mut self, subscript: Option<bool>) -> Self {
        self.subscript = subscript;
        self
    }

    pub fn character_spacing(mut self, character_spacing: Option<Length>) -> Self {
        self.character_spacing = character_spacing;
        self
    }

    pub fn width_scale(mut self, width_scale: Option<u32>) -> Self {
        self.width_scale = width_scale;
        self
    }

    pub fn position(mut self, position: Option<i32>) -> Self {
        self.position = position;
        self
    }

    pub fn hidden(mut self, hidden: Option<bool>) -> Self {
        self.hidden = hidden;
        self
    }
}

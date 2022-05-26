use bevy::prelude::*;

pub const DEFAULT_FONT: &str = r"fonts\OpenDyslexicMono-Regular.otf";

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub trait Menu: Component + Copy{
    fn text(&self) -> &str;

    fn size(&self, char_width: Option<f32>, char_height: Option<f32>) -> Size<Val>{
        let len = self.text().len() as f32;
        let width = char_width.unwrap_or(20.0);
        let height = char_height.unwrap_or(40.0);
        Size::new(Val::Px(width * len), Val::Px(height))
    }

    fn build(commands: &mut Commands, asset_server: Res<AssetServer>);

    fn to_button(&self, commands: &mut Commands, font: Handle<Font>) -> Entity{
        commands.spawn_bundle ( ButtonBundle {
            style: Style {
                size: self.size(None, None),
                margin: Rect::all(Val::Auto),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        }).with_children(|parent|{
            parent.spawn_bundle( TextBundle {
                text: Text::with_section(
                    self.text(),
                    TextStyle{
                        font,
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 0.5)
                    },
                    Default::default()
                ),
                ..Default::default()
            });
        })
        .insert(*self)
        .id()
        
    }
}
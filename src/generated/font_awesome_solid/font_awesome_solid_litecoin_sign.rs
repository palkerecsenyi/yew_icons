use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_litecoin_sign (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M128 195.3L247.2 161.2C264.2 156.4 281.9 166.2 286.8 183.2C291.6 200.2 281.8 217.9 264.8 222.8L128 261.9V416H352C369.7 416 384 430.3 384 448C384 465.7 369.7 480 352 480H96C78.33 480 64 465.7 64 448V280.1L40.79 286.8C23.8 291.6 6.087 281.8 1.232 264.8C-3.623 247.8 6.216 230.1 23.21 225.2L64 213.6V64C64 46.33 78.33 32 96 32C113.7 32 128 46.33 128 64V195.3z" /></ svg > } }
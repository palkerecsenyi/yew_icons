use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_bucket (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M96 160H48V152C48 68.05 116.1 0 200 0H248C331.9 0 400 68.05 400 152V160H352V152C352 94.56 305.4 48 248 48H200C142.6 48 96 94.56 96 152V160zM.0003 224C.0003 206.3 14.33 192 32 192H416C433.7 192 448 206.3 448 224C448 241.7 433.7 256 416 256H410.9L388.5 469C385.1 493.5 365.4 512 340.8 512H107.2C82.65 512 62.05 493.5 59.48 469L37.05 256H32C14.33 256 0 241.7 0 224H.0003z" /></ svg > } }
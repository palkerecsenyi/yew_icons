use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_child (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M224 64C224 99.35 195.3 128 160 128C124.7 128 96 99.35 96 64C96 28.65 124.7 0 160 0C195.3 0 224 28.65 224 64zM144 384V480C144 497.7 129.7 512 112 512C94.33 512 80.01 497.7 80.01 480V287.8L59.09 321C49.67 336 29.92 340.5 14.96 331.1C.0016 321.7-4.491 301.9 4.924 286.1L44.79 223.6C69.72 184 113.2 160 160 160C206.8 160 250.3 184 275.2 223.6L315.1 286.1C324.5 301.9 320 321.7 305.1 331.1C290.1 340.5 270.3 336 260.9 321L240 287.8V480C240 497.7 225.7 512 208 512C190.3 512 176 497.7 176 480V384L144 384z" /></ svg > } }
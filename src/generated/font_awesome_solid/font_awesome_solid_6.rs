use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_6 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M167.7 160.8l64.65-76.06c11.47-13.45 9.812-33.66-3.656-45.09C222.7 34.51 215.3 32.01 208 32.01c-9.062 0-18.06 3.833-24.38 11.29C38.07 214.5 0 245.5 0 320c0 88.22 71.78 160 160 160s160-71.78 160-160C320 234.4 252.3 164.9 167.7 160.8zM160 416c-52.94 0-96-43.06-96-96s43.06-95.1 96-95.1s96 43.06 96 95.1S212.9 416 160 416z" /></ svg > } }